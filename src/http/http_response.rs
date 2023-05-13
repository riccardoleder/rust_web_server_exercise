use crate::database;
use crate::file_system::read_file;
use std::collections::HashMap;
use std::path::Path;

pub fn generate_file_response(path: &Path) -> String {
    let body = read_file(path);
    let headers: HashMap<String, String> = HashMap::new();
    // headers.insert("Content-Length".to_string(), body.len().to_string());
    return generate_response(&HttpResponse {
        status: "HTTP/1.1 200 OK".to_string(),
        headers,
        body: Some(body),
    });
}

pub fn generate_not_found_response() -> String {
    let body = read_file(Path::new("pub/404.html"));
    let headers: HashMap<String, String> = HashMap::new();
    // headers.insert("Content-Length".to_string(), body.len().to_string());
    return generate_response(&HttpResponse {
        status: "HTTP/1.1 404 Not Found".to_string(),
        headers,
        body: Some(body),
    });
}

pub fn generate_internal_server_error_response() -> String {
    return generate_response(&HttpResponse {
        status: "HTTP/1.1 500 Internal Server Error".to_string(),
        headers: HashMap::new(),
        body: None,
    });
}

pub fn generate_forbidden_response() -> String {
    let body = read_file(Path::new("pub/403.html"));
    let headers: HashMap<String, String> = HashMap::new();
    // headers.insert("Content-Length".to_string(), body.len().to_string());
    return generate_response(&HttpResponse {
        status: "HTTP/1.1 403 Forbidden".to_string(),
        headers,
        body: Some(body),
    });
}

pub fn generate_redirect_response(redirect_path: &str) -> String {
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Location".to_string(), redirect_path.to_string());
    return generate_response(&HttpResponse {
        status: "HTTP/1.1 301 Moved Permanently".to_string(),
        headers,
        body: None,
    });
}

pub fn generate_data_response() -> String {
    let body = match database::read() {
        Err(_) => return generate_internal_server_error_response(),
        Ok(json) => json.to_string(),
    };
    let headers: HashMap<String, String> = HashMap::new();
    // headers.insert("Content-Length".to_string(), body.len().to_string());
    return generate_response(&HttpResponse {
        status: "HTTP/1.1 200 OK".to_string(),
        headers,
        body: Some(body),
    });
}

pub struct HttpResponse {
    status: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

pub fn generate_response(response: &HttpResponse) -> String {
    let mut headers = String::with_capacity(200);

    for (key, value) in response.headers.iter() {
        headers.push_str(format!("{key}: {value}\r\n").as_str())
    }

    match &response.body {
        None => return format!("{}\r\n{}", response.status, headers),
        Some(body) => return format!("{}\r\n{}\r\n\r\n{}", response.status, headers, body),
    }
}
