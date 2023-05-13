use super::http_status::{parse_http_method, parse_http_version, HttpMethod, HttpVersion};

pub struct HttpRequestLine {
    pub method: HttpMethod,
    pub target: String,
    pub version: HttpVersion,
}

impl HttpRequestLine {
    pub fn new(request_line: &String) -> Option<HttpRequestLine> {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        let method = match parse_http_method(parts[0]) {
            None => return None,
            Some(method) => method,
        };
        let target = parts[1].to_string();
        let version = match parse_http_version(parts[2]) {
            None => return None,
            Some(method) => method,
        };

        return Some(HttpRequestLine {
            method,
            target,
            version,
        });
    }
}
