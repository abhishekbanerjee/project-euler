use std::cmp;
use euler_rust::utils::files;
use euler_rust::utils::parse;

fn main() {
    let sum = max_path_sum_file("resources/0067_triangle.txt");
    println!("{}", sum);
}

fn max_path_sum_file(file_name: &str) -> u32 {
    max_path_sum(files::read_file(file_name).as_str())
}

// We parse the string into a 2-d grid of integers. Then we use
// dynamic programming to keep track of the maximum path starting at
// the current row. We build up from the bottom.
//
// This is the exact same method that we used for Problem 18.
fn max_path_sum(grid_str: &str) -> u32 {
    let mut grid = parse::parse_grid(grid_str);
    for i in (0..(grid.len()-1)).rev() {
	for j in 0..grid[i].len() {
	    grid[i][j] += cmp::max(grid[i+1][j], grid[i+1][j+1]);
	}
    }
    grid[0][0]
}

