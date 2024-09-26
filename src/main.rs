use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use crate::http_method::HttpMethod;
use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::http_status_code::HttpStatusCode;
use crate::http_version::HttpVersion;

mod http_method;
mod http_version;
mod http_request;
mod http_status_code;
mod http_response;

fn parse_request(raw: String) -> HttpRequest {
    let parts: Vec<&str> = raw.trim_end().split(' ').collect();
    let method = parts[0];
    let target = parts[1];
    let version = parts[2];
    //println!("|{}|{}|{}|", method, target, version);

    HttpRequest {
        method: method.parse::<HttpMethod>().unwrap(),
        target: String::from(target),
        version: version.parse::<HttpVersion>().unwrap(),
    }
}

fn root() -> HttpResponse {
    HttpResponse {
        version: HttpVersion::V11,
        status_code: HttpStatusCode::OK,
        headers: HashMap::default(),
        body: Vec::default(),
    }
}

fn echo(path: &str) -> HttpResponse {
    let mut contents: Vec<&str> = path.split('/').collect();
    let echo = contents.pop().unwrap();
    let len = echo.len();
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert(String::from("Content-Type"), String::from("text/plain"));
    headers.insert(String::from("Content-Length"), String::from(len.to_string()));
    HttpResponse {
        version: HttpVersion::V11,
        status_code: HttpStatusCode::OK,
        headers,
        body: Vec::from(echo),
    }
}

fn not_found() -> HttpResponse {
    HttpResponse {
        version: HttpVersion::V11,
        status_code: HttpStatusCode::NotFound,
        headers: HashMap::default(),
        body: Vec::default(),
    }
}

fn router(req: &HttpRequest) -> HttpResponse {
    match req.target.as_str() {
        "/" => root(),
        path if path.starts_with("/echo/") => echo(path),
        _ => not_found()
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let request = parse_request(buffer);
    let response = router(&request).to_string();
    let serialized = response.as_bytes();
    stream.write(serialized).unwrap();
}

fn main() {
    let addr = String::from("127.0.0.1:4221");
    let listener = TcpListener::bind(addr.clone()).unwrap();
    println!("Server listening on http://{}", addr);

    for incoming_bytes in listener.incoming() {
        match incoming_bytes {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
