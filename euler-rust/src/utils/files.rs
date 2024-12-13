use std::fs;

// Read a file at the given path to a String.
pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}
