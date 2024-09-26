use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::http_status_code::HttpStatusCode;
use crate::http_version::HttpVersion;
use std::collections::HashMap;

pub fn router(req: &HttpRequest) -> HttpResponse {
    match req.target.as_str() {
        "/" => root(),
        path if path.starts_with("/echo") => echo(req),
        path if path.starts_with("/user-agent") => user_agent(req),
        _ => not_found(),
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
    let mut contents: Vec<&str> = req.target.split('/').collect();
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

fn not_found() -> HttpResponse {
    HttpResponse {
        version: HttpVersion::V11,
        status_code: HttpStatusCode::NotFound,
        headers: HashMap::default(),
        body: Vec::default(),
    }
}
