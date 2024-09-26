use std::collections::HashMap;
use crate::http_method::HttpMethod;
use crate::http_version::HttpVersion;
use std::str::FromStr;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub target: String,
    pub version: HttpVersion,
    pub headers: HashMap<String, String>,
}

impl FromStr for HttpRequest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim_end().split("\r\n").collect();
        let request_line = parts[0];
        let request_items: Vec<&str> = request_line.split(' ').collect();
        let method = request_items[0].parse::<HttpMethod>()?;
        let target = String::from(request_items[1]);
        let version = request_items[2].parse::<HttpVersion>()?;
        let mut headers: HashMap<String, String> = HashMap::default();
        for part in parts.iter().skip(1) {
            let items: Vec<&str> = part.split(": ").collect();
            let key = String::from(items[0]);
            let value = String::from(items[1]);
            headers.insert(key, value);
        }
        Ok(Self { method, target, version, headers })
    }
}
