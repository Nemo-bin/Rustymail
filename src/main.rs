use std::{
    io::{BufReader, prelude::*},
    net::TcpListener,
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let http_request = recv(&mut stream);
    println!("{:#?}", http_request);
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!";
    send(&mut stream, response);
}

fn recv(stream: &mut std::net::TcpStream) -> Vec<String>{
    let buf_reader = BufReader::new(stream);
    let http_request: Vec<_> = buf_reader
        .lines() 
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    return http_request;
}

fn send(stream: &mut std::net::TcpStream, response: &str) {
    stream.write_all(response.as_bytes()).unwrap();
}
