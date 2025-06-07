use internal_service_health_check_api::ThreadPool;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&TcpStream> = BufReader::new(&stream);
    let mut lines: std::io::Lines<BufReader<&TcpStream>> = buf_reader.lines();
    let request_line: String = lines.next().unwrap().unwrap();
    let _http_request: Vec<_> = lines
        .by_ref()
        .map(|result: Result<String, std::io::Error>| result.unwrap())
        .take_while(|line: &String| !line.is_empty())
        .collect();
    
    let (status_line, content) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "Hello World!"),
        _ => ("HTTP/1.1 404 NOT FOUND", "ROUTE NOT FOUND"),
    };

    let length: usize = content.len();

    let response: String = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/plain\r\n\r\n{content}"
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener: TcpListener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let pool: ThreadPool = ThreadPool::new(4);
    
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}