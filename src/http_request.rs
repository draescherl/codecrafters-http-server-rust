use crate::http_method::HttpMethod;
use crate::http_version::HttpVersion;

pub struct HttpRequest {
    pub method: HttpMethod,
    pub target: String,
    pub version: HttpVersion,
}
