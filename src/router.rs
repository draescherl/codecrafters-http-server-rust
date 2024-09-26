use crate::http_method::HttpMethod;
use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::http_status_code::HttpStatusCode;
use crate::http_version::HttpVersion;
use crate::ServerConfig;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn router(server_config: &ServerConfig, req: &HttpRequest) -> HttpResponse {
    // dbg!(req);
    match req.path.as_str() {
        "/" => root(),
        path if path.starts_with("/echo") => echo(req),
        path if path.starts_with("/user-agent") => user_agent(req),
        path if path.starts_with("/files") => files(server_config, req),
        _ => HttpResponse::not_found(),
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

fn echo(req: &HttpRequest) -> HttpResponse {
    let mut contents: Vec<&str> = req.path.split('/').collect();
    let echo = contents.pop().unwrap();
    let content_length = echo.len();
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert(String::from("Content-Type"), String::from("text/plain"));
    headers.insert(String::from("Content-Length"), content_length.to_string());
    HttpResponse {
        version: HttpVersion::V11,
        status_code: HttpStatusCode::OK,
        headers,
        body: Vec::from(echo),
    }
}

fn user_agent(req: &HttpRequest) -> HttpResponse {
    let user_agent = req.headers.get("User-Agent").unwrap();
    let content_length = user_agent.len();
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert(String::from("Content-Type"), String::from("text/plain"));
    headers.insert(String::from("Content-Length"), content_length.to_string());
    HttpResponse {
        version: HttpVersion::V11,
        status_code: HttpStatusCode::OK,
        headers,
        body: Vec::from(user_agent.clone()),
    }
}

fn files_get(path: String) -> HttpResponse {
    let reading = fs::read_to_string(path);
    match reading {
        Ok(contents) => {
            let content_length = contents.len();
            let mut headers: HashMap<String, String> = HashMap::new();
            headers.insert(
                String::from("Content-Type"),
                String::from("application/octet-stream"),
            );
            headers.insert(String::from("Content-Length"), content_length.to_string());
            HttpResponse {
                version: HttpVersion::V11,
                status_code: HttpStatusCode::OK,
                headers,
                body: Vec::from(contents),
            }
        }
        Err(_) => HttpResponse::not_found(),
    }
}

fn files_post(path: String, contents: &[u8]) -> HttpResponse {
    let mut file = File::create(path).unwrap();
    file.write_all(contents).unwrap();
    HttpResponse {
        version: HttpVersion::V11,
        status_code: HttpStatusCode::Created,
        headers: HashMap::new(),
        body: Vec::default(),
    }
}

fn files(server_config: &ServerConfig, req: &HttpRequest) -> HttpResponse {
    let directory = server_config.directory.clone().unwrap();
    let filename = req.path.split('/').collect::<Vec<&str>>()[2];
    let path = format!("{}/{}", directory, filename);
    match req.method {
        HttpMethod::Get => files_get(path),
        HttpMethod::Post => files_post(path, req.body),
    }
}
