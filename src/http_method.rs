use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum HttpMethod {
    Get,
    Post,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
        };
        write!(f, "{}", str)
    }
}

impl FromStr for HttpMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            _ => Err(format!("Could not parse [{}] into an HttpMethod.", s)),
        }
    }
}
