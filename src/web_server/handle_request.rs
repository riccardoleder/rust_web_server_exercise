use crate::http::http_request_line::HttpRequestLine;
use crate::http::http_response::{
    generate_data_response, generate_file_response, generate_forbidden_response,
    generate_internal_server_error_response, generate_not_found_response,
    generate_redirect_response,
};
use crate::http::http_status::HttpMethod;
use crate::{log, persistance};
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::path::Path;

/// Handles a request and generates the correct response as a String
pub fn handle_request(stream: &TcpStream) -> String {
    let mut reader = BufReader::new(stream);
    let mut buffer = String::with_capacity(1000);

    log!("Read request line");
    // Read and process the request line
    let request_line_length = reader.read_line(&mut buffer);
    if request_line_length.is_err() {
        return generate_not_found_response();
    }

    log!("Parse request line");
    let request_line = match HttpRequestLine::new(&buffer) {
        Some(line) => line,
        None => return generate_not_found_response(),
    };
    buffer.clear();

    log!("Parse headers");
    // Read and process the headers
    let mut headers = HashMap::new();
    loop {
        let header_line_length = reader.read_line(&mut buffer);

        if header_line_length.is_err() {
            return generate_not_found_response();
        }

        if buffer.is_empty() || buffer == "\r\n" {
            break; // End of headers, break the loop
        }

        let parts: Vec<&str> = buffer.splitn(2, ": ").collect();
        headers.insert(parts[0].to_string(), parts[1].to_string());
        buffer.clear();
    }
    buffer.clear();

    log!("Parse body");
    // Read and process the request body (if any)
    let _body_size = reader.read_to_string(&mut buffer);
    // if body_size.is_err() {
    //     log!("{}", body_size.unwrap_err());
    //     return generate_fallback_response();
    // }

    // Request parsed, now let's respond
    let request = format!(
        "{:?} {} {:?}\n\n{:#?} \n\n{}",
        request_line.method, request_line.target, request_line.version, headers, buffer
    );
    log!("Request parsed: \n{request}");

    match request_line.method {
        HttpMethod::GET => {
            if request_line.target == "/data" {
                return generate_data_response();
            }
            // if request_line.target.starts_with("/data/") {
            //     let rec = request_line.target.to_string().split_off(6);
            //     println!("rec: {}", rec)
            // }

            if request_line.target == "/" {
                return generate_redirect_response("/index.html");
            }

            let requested_path = format!("pub{}", request_line.target);
            let absolute_path = match Path::new(requested_path.as_str()).canonicalize() {
                Err(e) => {
                    log!("error canonicalize: {}", e);
                    return generate_not_found_response();
                }
                Ok(path) => path,
            };

            let pub_path = match Path::new("pub").canonicalize() {
                Err(_) => {
                    return generate_forbidden_response();
                }
                Ok(path) => path,
            };

            log!("canonicalized path: {}", absolute_path.to_str().unwrap());

            if !absolute_path.starts_with(&pub_path) | absolute_path.is_dir() {
                return generate_forbidden_response();
            }

            //Elaborate a response
            return generate_file_response(absolute_path.as_path());
        }
        HttpMethod::POST => {
            if request_line.target == "/data" {
                println!("string json to write: \r\n{}", &buffer);

                let json = match serde_json::from_str(&buffer) {
                    Err(_) => return generate_internal_server_error_response(),
                    Ok(val) => val,
                };

                println!("buffer json to write: \r\n{}", json);

                match persistance::write(&json) {
                    Err(_) => return generate_internal_server_error_response(),
                    Ok(_) => return generate_data_response(),
                }
            }
            // if request_line.target.starts_with("/data/") {
            //     let rec = request_line.target.to_string().split_off(6);
            //     println!("rec: {}", rec)
            // }

            return generate_internal_server_error_response();
        }
        _ => return generate_not_found_response(),
    }
}
