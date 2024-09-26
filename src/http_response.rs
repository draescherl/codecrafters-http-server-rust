use crate::http_status_code::HttpStatusCode;
use crate::http_version::HttpVersion;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct HttpResponse {
    pub version: HttpVersion,
    pub status_code: HttpStatusCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn not_found() -> Self {
        Self {
            version: HttpVersion::V11,
            status_code: HttpStatusCode::NotFound,
            headers: HashMap::default(),
            body: Vec::default(),
        }
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = self.version.to_string();
        str.push(' ');
        str.push_str(&self.status_code.to_string());
        str.push_str("\r\n");
        for (key, value) in &self.headers {
            let tmp = format!("{}: {}\r\n", key, value);
            str.push_str(&tmp);
        }
        str.push_str("\r\n");
        str.push_str(&String::from_utf8_lossy(&self.body));
        write!(f, "{}", str)
    }
}

impl Default for HttpResponse {
    fn default() -> Self {
        let mut headers: HashMap<String, String> = HashMap::new();
        let body: Vec<u8> = Vec::from("Default response, should only be used for debugging.");
        let content_length = body.len();
        headers.insert(String::from("Content-Type"), String::from("text/plain"));
        headers.insert(String::from("Content-Length"), content_length.to_string());
        Self {
            version: HttpVersion::V11,
            status_code: HttpStatusCode::OK,
            headers,
            body,
        }
    }
}
