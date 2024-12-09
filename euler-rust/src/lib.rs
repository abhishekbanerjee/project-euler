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

// Sums up all the divisors of a number except for itself.
pub fn divisors_sum(n: u32) -> u32 {
    if n == 1 { return 0; }
    // The square root is the largest possible non-identity divisor of
    // a number.
    let root = (n as f64).sqrt().ceil() as u32;
    // Start the count at 1, because 1 always divides the
    // number.
    let mut divisors_sum = 1u32;
    for d in 2..root {
	// Each number d evenly dividing n gets two distinct divisors:
	// d and n/d. Since d < sqrt(n) by the loop condition,
	// therefore n/d > sqrt(n) and we therefore would not
	// encounter n/d in this loop.
	if n % d == 0 {
	    divisors_sum += d + n/d;
	}
    }
    // If n is an even square, the square root counts only once.
    if n == root*root {
	divisors_sum += root;
    }
    divisors_sum
}
