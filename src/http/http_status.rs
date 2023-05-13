#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    PATCH,
}

#[derive(Debug)]
pub enum HttpVersion {
    Http10,
    Http11,
    Http20,
}

pub fn parse_http_method(method: &str) -> Option<HttpMethod> {
    match method {
        "GET" => Some(HttpMethod::GET),
        "POST" => Some(HttpMethod::POST),
        "PUT" => Some(HttpMethod::PUT),
        "DELETE" => Some(HttpMethod::DELETE),
        "HEAD" => Some(HttpMethod::HEAD),
        "OPTIONS" => Some(HttpMethod::OPTIONS),
        "CONNECT" => Some(HttpMethod::CONNECT),
        "TRACE" => Some(HttpMethod::TRACE),
        "PATCH" => Some(HttpMethod::PATCH),
        _ => None,
    }
}

pub fn parse_http_version(version: &str) -> Option<HttpVersion> {
    match version {
        "HTTP/1.0" => Some(HttpVersion::Http10),
        "HTTP/1.1" => Some(HttpVersion::Http11),
        "HTTP/2.0" => Some(HttpVersion::Http20),
        _ => None,
    }
}
