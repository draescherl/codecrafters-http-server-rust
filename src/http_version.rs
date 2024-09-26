use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum HttpVersion {
    V11,
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            HttpVersion::V11 => "HTTP/1.1",
        };
        write!(f, "{}", str)
    }
}

impl FromStr for HttpVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(HttpVersion::V11),
            _ => Err(format!("Could not parse [{}] into an HttpVersion.", s)),
        }
    }
}
