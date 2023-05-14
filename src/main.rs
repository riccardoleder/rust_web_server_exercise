use rust_web_server_exercise::log;
use rust_web_server_exercise::logger::save_log;
pub use rust_web_server_exercise::logger::write_to_log;
use rust_web_server_exercise::web_server::setup::setup_server;
use std::io;
use std::path::Path;
use std::thread;

fn main() {
    log!("Main starting...");
    // Spawn server in a new thread
    let child: thread::JoinHandle<()> = thread::spawn(|| listener());

    setup_server();

    child.join().unwrap();

    save_log(Path::new("data/log.txt"));
}

/// Listen for "shutdown" command input from the terminal
/// TODO: improve to manage ctrlc signals
fn listener() {
    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.contains("shutdown") {
                    log!("shutting down server...");
                    unsafe { rust_web_server_exercise::web_server::setup::STOP_SERVER = true };
                    break;
                } else {
                    log!("write 'shutdown' to terminate server");
                }
            }
            Err(error) => {
                log!("{error}");
            }
        }

        input.clear()
    }
}
