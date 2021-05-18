use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

const BIND_ADDRESS: &str = "127.0.0.1:8080";

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Connection established");
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer[..]);

    println!("Received something:\n{}", request);

    let request_parts: Vec<&str> = request.splitn(2, "\r\n").collect();

    let request_line = request_parts.get(0);
    if request_line.is_none() {
        println!("No request line? Throwing 400");
        let response = "HTTP/1.1 400 Bad Request\r\n\r\n";
        stream.write(response.as_bytes())?;
        return Ok(());
    }

    let headers_and_body = request_parts
        .get(1)
        .map(|headers_and_body_content| headers_and_body_content.splitn(2, "\r\n\r\n").collect::<Vec<&str>>())
        .unwrap_or(vec!());
    let (headers, body) = (headers_and_body.get(0), headers_and_body.get(1));

    println!("Received request: {}", request_line.filter(|request_content| request_content.trim().len() > 0).unwrap_or(&"<no request>"));
    println!("Received headers: {}", headers.filter(|headers_content| headers_content.trim().len() > 0).unwrap_or(&"<no headers>"));
    println!("Received body: {}", body.filter(|body_content| body_content.trim().len() > 0).unwrap_or(&"<no body>"));

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes())?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Starting server on '{}'", BIND_ADDRESS);

    let listener = TcpListener::bind(BIND_ADDRESS)?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
