use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::path::Path;

static HOST: &str = "localhost";
static PORT: &str = "8080";

fn main() {
    let address = format!("{}:{}", HOST, PORT);
    let listener = TcpListener::bind(&address)
        .expect(&format!("Failed to bind to: {}", &address));

    println!("Listening on: {}", &address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => panic!("Error! {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let request =  String::from_utf8_lossy(&buffer[..]);

    println!("Request: {}", &request);

    let lines: Vec<&str> = request.split("\r\n").collect();
    /*
    Method Request-URI HTTP-Version CRLF
    headers CRLF
    message-body CRLF
    CRLF
    */
    let first_line_parts: Vec<&str> = lines.get(0).unwrap().split(" ").collect();
    let method = first_line_parts.get(0).unwrap();
    let req_uri = first_line_parts.get(1).unwrap();

    println!("Requested: METHOD={}, URI={}", &method, &req_uri);

    let static_file_prefix: &Path = Path::new("static");
    let uri = static_file_prefix.join(
            match *req_uri {
            "/" => "index.html",
            _ if req_uri.starts_with("/") => &req_uri[1..],
            _ => &req_uri
        }
    );

    println!("URI: {:?}", &uri);

    let response_content = fs::read_to_string(uri).ok();

    println!("Content: {:?}", &response_content);

    let response = match response_content {
        Some(content) => format!("HTTP/1.1 200 OK \r\n\r\n{}", content),
        None => String::from("HTTP/1.1 404 OK \r\n\r\n<html><body><h1>Not Found</h1></body></html>"),
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
