use chrono::{DateTime, Utc};
use serde_json::Value;
use std::{fs, io, str::FromStr};

pub const DATA_PATH: &str = "data/data.json";

pub enum DatabaseError {
    IOError(io::Error),
    SerdeError(serde_json::Error),
}

// TODO: Manage concurrency
/// Reads all the persisted data
pub fn read() -> io::Result<Value> {
    let data = match read_raw() {
        Err(e) => return Err(e),
        Ok(val) => val,
    };

    match data.get("data") {
        None => return Err(io::Error::new(io::ErrorKind::Other, "corrupted data")),
        Some(val) => return Ok(val.to_owned()),
    };
}

pub fn read_raw() -> io::Result<Value> {
    let data = fs::read_to_string(DATA_PATH);
    match data {
        Err(e) => return Err(e),
        Ok(data) => {
            let json_result = serde_json::Value::from_str(data.as_str());
            match json_result {
                Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
                Ok(json) => return Ok(json),
            }
        }
    }
}

// TODO: Manage concurrency
/// Writes persistent data
pub fn write(json: &Value) -> io::Result<()> {
    let new_mod = Utc::now();

    let current_json = match read_raw() {
        Err(e) => return Err(e),
        Ok(val) => val,
    };

    let latest_mod = match get_mod(&current_json) {
        Err(e) => return Err(e),
        Ok(val) => val,
    };

    if new_mod <= latest_mod {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "cannot write to a newer mod",
        ));
    }

    let mut json_data = serde_json::Map::new();
    json_data.insert(
        "mod".to_string(),
        serde_json::Value::String(new_mod.to_rfc3339()),
    );
    json_data.insert("data".to_string(), json.to_owned());

    let json_data = serde_json::Value::Object(json_data);
    let json_pretty = match serde_json::to_string_pretty(&json_data) {
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "can't convert to pretty json",
            ))
        }
        Ok(val) => val,
    };

    match fs::write(DATA_PATH, json_pretty) {
        Err(e) => return Err(e),
        Ok(_) => return Ok(()),
    };
}

pub fn get_mod(json: &Value) -> io::Result<DateTime<Utc>> {
    let mod_timestamp = match json.get("mod") {
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "data file corrupted",
            ))
        }
        Some(val) => val,
    };

    let mod_timestamp = match mod_timestamp.as_str() {
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "data file corrupted",
            ))
        }
        Some(val) => val,
    };

    let mod_timestamp = match chrono::DateTime::from_str(mod_timestamp) {
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "data file corrupted",
            ))
        }
        Ok(val) => val,
    };

    return Ok(mod_timestamp);
}
