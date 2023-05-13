use super::handle_request::handle_request;
use crate::log;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub static mut STOP_SERVER: bool = false;

/// Starts server
pub fn setup_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    log!("Server started at 127.0.0.1:7878");

    while !unsafe { STOP_SERVER } {
        // Check for incoming connections
        match listener.accept() {
            Ok((stream, _)) => {
                // Handle the connection in a new thread
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No connections available, sleep for a bit
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(e) => {
                // Handle other errors
                log!("Error accepting connection: {e}");
            }
        }
    }

    log!("Server shutdown");
}

/// Handles an upcoming connection
fn handle_connection(mut stream: TcpStream) {
    log!("Handling connection");

    let response = handle_request(&stream);

    // Respond to client
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.shutdown(std::net::Shutdown::Both).unwrap();
}
