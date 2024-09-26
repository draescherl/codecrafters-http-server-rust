use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::http_status_code::HttpStatusCode;
use crate::http_version::HttpVersion;

#[derive(Clone)]
pub struct HttpResponse {
    pub version: HttpVersion,
    pub status_code: HttpStatusCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
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
