use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use crate::http_request::HttpRequest;
use crate::router::router;

mod http_method;
mod http_request;
mod http_response;
mod http_status_code;
mod http_version;
mod router;

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub directory: Option<String>,
}

async fn handle_connection(server_config: ServerConfig, mut stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut request_buffer = String::new();
    reader.read_line(&mut request_buffer).unwrap();
    while request_buffer.chars().rev().take(4).collect::<String>() != "\n\r\n\r" {
        reader.read_line(&mut request_buffer).unwrap();
    }
    let request = request_buffer.parse::<HttpRequest>().unwrap();
    let response = router(&server_config, &request).to_string();
    let serialized = response.as_bytes();
    stream.write_all(serialized).unwrap();
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut config = ServerConfig { directory: None };
    if args.len() == 3 && args[1] == "--directory" {
        config = ServerConfig { directory: Some(args[2].clone()) }
    }

    let addr = String::from("127.0.0.1:4221");
    let listener = TcpListener::bind(addr.clone()).unwrap();
    println!("Server listening on http://{}.", addr);

    for incoming_bytes in listener.incoming() {
        match incoming_bytes {
            Ok(stream) => {
                let peer = stream.peer_addr().unwrap();
                println!("Incoming connection from [{}] accepted.", peer);
                tokio::spawn(handle_connection(config.clone(), stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
