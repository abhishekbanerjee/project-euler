use euler_rust::utils::files;
use euler_rust::utils::parse;
use std::cmp;

fn main() {
    let sum = path_sum_two_file("resources/0081_matrix.txt");
    println!("{}", sum);
}

fn path_sum_two_file(file_path: &str) -> u32 {
    path_sum_two(files::read_file(file_path).as_str())
}

// We solve this using dynamic programming, very similarly to the
// solution for Problem 15. The key observation is that we can get to
// (i, j) either through (i-1, j) or (i, j-1), so we minimize the path
// to get to these previous points and choose the optimal path to
// (i,j). Another observation from that problem is that to populate
// the current row, we only need data from the previous row and none
// of the ones further back, so that is all we keep.
fn path_sum_two(text: &str) -> u32 {
    let grid = parse::parse_grid(text, ",");
    let (m, n) = (grid.len(), grid[0].len());
    let mut row_sums = vec![grid[0][0]];
    for j in 1..n { row_sums.push(grid[0][j] + row_sums[j-1]); }
    for i in 1..m {
	let mut new_row_sums = vec![row_sums[0] + grid[i][0]];
	for j in 1..n {
	    new_row_sums.push(grid[i][j] + cmp::min(new_row_sums[j-1], row_sums[j]));
	}
	row_sums = new_row_sums;
    }
    row_sums[n-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_sum_two() {
	let text = "\
	131,673,234,103,18\n\
	201,96,342,965,150\n\
	630,803,746,422,111\n\
	537,699,497,121,956\n\
	805,732,524,37,331\
	";
	assert_eq!(path_sum_two(text), 2427);
    }
}
