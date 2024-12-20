use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;

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

// Combines a slice of objects to a single string formed from the
// concatenation of their string forms.
pub fn parse_slice_as_string<T: ToString>(slice: &[T]) -> String {
    slice.into_iter().map(|d| d.to_string()).collect::<String>()
}

// Combines a slice of numbers (typically digits) into a single number
// corresponding to them all written out together.
pub fn parse_slice_as_number<T: ToString, U: FromStr<Err : Debug>>(digits: &[T]) -> U {
    parse_slice_as_string(digits).parse::<U>().unwrap()
}

// Splits the digits of the number.
pub fn split_number_to_digits<T: ToString, U: FromStr<Err: Debug>>(n: T) -> Vec<U> {
    n.to_string().chars().into_iter()
	.map(|c| c.to_string().parse::<U>().unwrap())
	.collect()
}

// Extracts the first digit of the given number.
pub fn first_digit<T: Display + FromStr<Err: Debug> + ToString>(n: T) -> T {
    nth_digit(n, 0usize)
}

// Extracts the n-th (0-based) digit of the given number (written from
// left to right).
pub fn nth_digit<T: Display + FromStr<Err: Debug> + ToString>(number: T, idx: usize) -> T {
    number
	.to_string()[(idx)..(idx+1)]
	.parse::<T>()
	.expect(format!("{} does not have a {}-th digit!", number, idx).as_str())
}
