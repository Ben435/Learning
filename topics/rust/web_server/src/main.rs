use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

mod http_request;
mod hash_multi_map;

use http_request::{HttpRequest,HttpRequestMethod};
use hash_multi_map::HashMultiMap;

const BIND_ADDRESS: &str = "127.0.0.1:8080";

fn construct_http_request<'a>(request: &'a str) -> Option<HttpRequest<'a>> {
    println!("Received something:\n{}", request);

    let request_parts: Vec<&str> = request.splitn(2, "\r\n").collect();

    let request_line = request_parts.get(0);
    if request_line.is_none() {
        println!("No request line?");
        return None;
    }

    let line_parts: Vec<&str> = request_line
        .filter(|request_content| request_content.trim().len() > 0)
        .map(|line| line.split(" ").collect())
        .unwrap_or_default();
    
    let request_method = line_parts
        .get(0)
        .map(|method| HttpRequestMethod::from_str(method))
        .flatten()
        .unwrap();
    
    let request_path = line_parts
        .get(1)
        .unwrap();

    let headers_and_body = request_parts
        .get(1)
        .map(|headers_and_body_content| headers_and_body_content.splitn(2, "\r\n\r\n").collect::<Vec<&str>>())
        .unwrap_or(vec!());
    let (headers, body) = (headers_and_body.get(0).unwrap(), headers_and_body.get(1).unwrap());

    let headers = headers
        .split("\n")
        .map(|header_line| header_line.split_once(":").unwrap())
        .fold(HashMultiMap::new(), |mut headers_map, (header_key, header_val)| {
            headers_map.push_to_key(&String::from(header_key.trim()), header_val.trim());

            headers_map
        });

    println!("Received request: '{} {}'", request_method, request_path);
    println!("Received User-Agent: {:?}", headers.get(&String::from("User-Agent")));
    println!("Received body: {}", body);

    Some(HttpRequest {
        request_method,
        request_path,
        headers,
        body,
    })
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Connection established");
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer[..]);

    let request_obj = construct_http_request(request.as_ref()).unwrap();

    println!("Request: {} {}", request_obj.request_method, request_obj.request_path);

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
