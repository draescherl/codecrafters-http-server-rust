use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::http_request::HttpRequest;
use crate::router::router;

mod http_headers;
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
    let mut buffer = vec![0; 2048];
    let amount = stream.read(&mut buffer).unwrap();
    let request = HttpRequest::from_bytes(&buffer[..amount]).unwrap();
    let response = router(&server_config, &request).to_string();
    let serialized = response.as_bytes();
    stream.write_all(serialized).unwrap();
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut config = ServerConfig { directory: None };
    if args.len() == 3 && args[1] == "--directory" {
        config = ServerConfig {
            directory: Some(args[2].clone()),
        }
    }

    let addr = "127.0.0.1:4221";
    let listener = TcpListener::bind(addr).unwrap();
    println!("Server listening on http://{}.", addr);
    loop {
        let (stream, socket_addr) = listener.accept().unwrap();
        println!("Incoming connection from [{}] accepted.", socket_addr);
        tokio::spawn(handle_connection(config.clone(), stream));
    }
}
