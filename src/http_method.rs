use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub enum HttpMethod {
    GET,
    POST,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            HttpMethod::GET => { "GET" }
            HttpMethod::POST => { "POST" }
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug)]
pub struct HttpMethodParseError;

impl FromStr for HttpMethod {
    type Err = HttpMethodParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            _ => Err(HttpMethodParseError)
        }
    }
}
