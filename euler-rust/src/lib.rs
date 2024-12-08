use std::fs;

/**
 * Common library functions needed for all the programs in the
 * package.
 */

// Read a file at the given path to a String.
pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

// Parse a multiline string of space separated integers into a 2-D
// vector.
pub fn parse_grid(grid_str: &str) -> Vec<Vec<u32>> {
    let mut grid = Vec::new();
    for line in grid_str.lines() {
	let mut row = Vec::new();
	for number in line.split_whitespace() {
	    row.push(number.parse::<u32>().unwrap());
	}
	grid.push(row);
    }
    grid
}
