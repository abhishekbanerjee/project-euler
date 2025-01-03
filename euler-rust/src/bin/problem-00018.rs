use std::cmp;
use euler_rust::utils::parse;

fn main() {
    let sum = max_path_sum("\
	75\n\
	95 64\n\
	17 47 82\n\
	18 35 87 10\n\
	20 04 82 47 65\n\
	19 01 23 75 03 34\n\
	88 02 77 73 07 63 67\n\
	99 65 04 28 06 16 70 92\n\
	41 41 26 56 83 40 80 70 33\n\
	41 48 72 33 47 32 37 16 94 29\n\
	53 71 44 65 25 43 91 52 97 51 14\n\
	70 11 33 28 77 73 17 78 39 68 17 57\n\
	91 71 52 38 17 14 91 43 58 50 27 29 48\n\
	63 66 04 68 89 53 67 30 73 16 69 87 40 31\n\
	04 62 98 27 23 09 70 98 73 93 38 53 60 04 23\
	");
    println!("{}", sum);
}

// We parse the string into a 2-d grid of integers. Then we use
// dynamic programming to keep track of the maximum path starting at
// the current row. We build up from the bottom.
fn max_path_sum(grid_str: &str) -> u32 {
    let mut grid = parse::parse_grid(grid_str, " ");
    for i in (0..(grid.len()-1)).rev() {
	for j in 0..grid[i].len() {
	    grid[i][j] += cmp::max(grid[i+1][j], grid[i+1][j+1]);
	}
    }
    grid[0][0]
}

