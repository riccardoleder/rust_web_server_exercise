use chrono::Utc;
use std::{fs, path::Path};

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        {
            let mut message: String = format!($($arg)*);
            println!("{}", message);
            message.push_str("\r\n");
            crate::write_to_log(&message);
        }
    };
}

static mut LOG: String = String::new();

pub fn write_to_log(msg: &String) {
    unsafe { LOG.push_str(msg) };
}

pub fn read_log() -> &'static str {
    return unsafe { LOG.as_str() };
}

pub fn save_log(path: &Path) {
    unsafe {
        LOG.insert_str(
            0,
            format!("LOG - {}\r\n---\r\n\r\n", generate_timestamp()).as_str(),
        )
    };
    match fs::write(path, read_log()) {
        Err(_) => println!("could NOT save log to {}", path.to_str().unwrap_or("---")),
        Ok(_) => {
            println!("log saved to {}", path.to_str().unwrap_or("---"))
        }
    };
}

fn generate_timestamp() -> String {
    return Utc::now().to_rfc3339();
}
