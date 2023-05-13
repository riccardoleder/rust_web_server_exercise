use rust_web_server_exercise;

use rust_web_server_exercise::web_server::setup::setup_server;
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use std::thread;
use std::time::Duration;

#[test]
fn test_setup_server() {
    println!("Testing server setup");

    // Spawn server in a new thread
    let child: thread::JoinHandle<()> = thread::spawn(|| setup_server());

    // Wait for server to setup
    thread::sleep(Duration::from_secs(2));

    println!("Sending request...");

    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let request = "GET /index.html HTTP/1.1\r\n\
                   Host: www.example.com\r\n\
                   Connection: close\r\n\
                   User-Agent: Rust HTTP\r\n\
                   Accept-Encoding: gzip\r\n\
                   \r\n";

    stream.write_all(request.as_bytes()).unwrap();
    stream.flush().unwrap();

    // Read the server's response
    let mut buffer = Vec::new();
    match stream.read_to_end(&mut buffer) {
        Ok(_) => println!("Successfully read response"),
        Err(e) => println!("Failed to read response: {e}"),
    }

    let response = String::from_utf8_lossy(&buffer[..]).to_string();

    println!("response received: \n{response}");
    assert!(
        response.contains("HTTP/1.1 200 OK"),
        "response not ok, actual response: {response}"
    );

    stream.shutdown(Shutdown::Both).unwrap();

    stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let request2 = "GET /../src/main.rs HTTP/1.1\r\n\
                   Host: www.example.com\r\n\
                   Connection: close\r\n\
                   User-Agent: Rust HTTP\r\n\
                   Accept-Encoding: gzip\r\n\
                   \r\n";

    stream.write_all(request2.as_bytes()).unwrap();

    // Read the server's response
    let mut buffer = Vec::new();
    match stream.read_to_end(&mut buffer) {
        Ok(_) => println!("Successfully read response"),
        Err(e) => println!("Failed to read response: {e}"),
    }

    let response = String::from_utf8_lossy(&buffer[..]).to_string();

    println!("response received: \n{response}");
    assert!(
        response.contains("HTTP/1.1 403 Forbidden"),
        "response not ok, actual response: {response}"
    );

    // Wait for the new thread to finish
    // kill server thread
    unsafe { rust_web_server_exercise::web_server::setup::STOP_SERVER = true }
    child.join().unwrap();

    // Code after the join call is executed after the new thread has finished
    println!("Test teardown!");
}
