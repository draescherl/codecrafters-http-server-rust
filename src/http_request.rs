use crate::http_headers::HttpHeaders;
use crate::http_method::HttpMethod;
use crate::http_version::HttpVersion;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest<'a> {
    pub method: HttpMethod,
    pub path: String,
    pub version: HttpVersion,
    pub headers: HttpHeaders,
    pub body: &'a [u8],
}

impl HttpRequest<'_> {
    pub fn from_bytes(input: &[u8]) -> Result<HttpRequest, String> {
        let request_line_end = input
            .windows(2)
            .position(|arr| arr == b"\r\n")
            .ok_or("Malformed request: could not find request line/header separator.")?;
        let headers_end = input[request_line_end + 2..]
            .windows(4)
            .position(|arr| arr == b"\r\n\r\n")
            .ok_or("Malformed request: could not find body separator.")?
            + request_line_end
            + 2;

        let (method, path, version) = Self::parse_request_line(&input[..request_line_end])?;
        let headers = Self::parse_headers(&input[request_line_end + 2..headers_end + 2])?;
        let body = &input[headers_end + 4..];
        Ok(HttpRequest {
            method,
            path,
            version,
            headers,
            body,
        })
    }

    fn parse_request_line(input: &[u8]) -> Result<(HttpMethod, String, HttpVersion), String> {
        let input_as_string = String::from_utf8_lossy(input);
        let request_items: Vec<&str> = input_as_string.split(' ').collect();
        let method = request_items[0].parse::<HttpMethod>()?;
        let path = String::from(request_items[1]);
        let version = request_items[2].parse::<HttpVersion>()?;
        Ok((method, path, version))
    }

    fn parse_headers(input: &[u8]) -> Result<HttpHeaders, String> {
        fn inner(input: &[u8], mut headers: HttpHeaders) -> Result<HttpHeaders, String> {
            if input.is_empty() || input == b"\r\n" {
                Ok(headers)
            } else {
                let separator = input
                    .windows(2)
                    .position(|arr| arr == b": ")
                    .ok_or("Malformed request: could not find header key/value separator")?;
                let header_end = input[separator + 2..]
                    .windows(2)
                    .position(|arr| arr == b"\r\n")
                    .ok_or("Malformed request: could not find header/header separator.")?
                    + separator
                    + 2;
                let key = String::from_utf8_lossy(&input[..separator]).to_string();
                let value = String::from_utf8_lossy(&input[separator + 2..header_end]).to_string();
                let _ = headers.insert(key, value);
                inner(&input[header_end + 2..], headers)
            }
        }
        inner(input, HashMap::new())
    }
}
