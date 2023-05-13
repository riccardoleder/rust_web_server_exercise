use std::{fs, path::Path};

pub fn read_file(path: &Path) -> String {
    return fs::read_to_string(path).unwrap();
}
