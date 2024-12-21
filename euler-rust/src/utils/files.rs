use std::fs;

// Read a file at the given path to a String.
pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

// Parse a list of words from the file where they are comma-separated
// and within quotes.
pub fn parse_words(file_path: &str) -> Vec<String> {
    read_file(file_path).split(",").map(|s| {
	let mut s_str = s.to_string();
	s_str.pop(); // Remove last character.
	if s_str.len() > 0 { s_str.remove(0); } // Remove first character.
	s_str
    }).collect()
}
