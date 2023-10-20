use learn_webserver::thread_pool;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

static WORKER_THREADS: usize = 4;

static HOST: &str = "localhost";
static PORT: &str = "8080";

static NOT_FOUND_FILE: &str = "404.html";

fn main() {
    let pool = thread_pool::ThreadPool::new(WORKER_THREADS).unwrap();

    let address = format!("{}:{}", HOST, PORT);
    let listener = TcpListener::bind(&address).expect(&format!("Failed to bind to: {}", &address));

    println!("Listening on: {}", &address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(|| handle_connection(stream)),
            Err(e) => panic!("Error! {}", e),
        }
    }

    println!("Shutting down!")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);

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
    let uri = static_file_prefix.join(match *req_uri {
        "/" => "index.html",
        _ if req_uri.starts_with("/") => &req_uri[1..],
        _ => &req_uri,
    });

    let response_content = fs::read_to_string(uri);

    println!("Content: {:?}", &response_content);

    let response = match response_content {
        Ok(content) => format!("HTTP/1.1 200 OK \r\n\r\n{}", content),
        Err(_) => format!("HTTP/1.1 404 OK \r\n\r\n{}", &NOT_FOUND_FILE),
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
