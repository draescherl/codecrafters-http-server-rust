use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone)]
pub enum HttpStatusCode {
    OK,
    Created,
    NotFound,
    InternalServerError,
}

impl Display for HttpStatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            HttpStatusCode::OK => "200 OK",
            HttpStatusCode::Created => "201 Created",
            HttpStatusCode::NotFound => "404 Not Found",
            HttpStatusCode::InternalServerError => "500 Internal Server Error",
        };
        write!(f, "{}", str)
    }
}

impl FromStr for HttpStatusCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "200 OK" => Ok(HttpStatusCode::OK),
            "201 Created" => Ok(HttpStatusCode::Created),
            "404 Not Found" => Ok(HttpStatusCode::NotFound),
            "500 Internal Server Error" => Ok(HttpStatusCode::InternalServerError),
            _ => Err(format!("Could not parse [{}] into an HttpStatusCode.", s)),
        }
    }
}
