use euler_rust::utils::files;
use euler_rust::utils::parse;
use std::cmp;

fn main() {
    let sum = path_sum_three_file("resources/0082_matrix.txt");
    println!("{}", sum);
}

fn path_sum_three_file(file_path: &str) -> u32 {
    path_sum_three(files::read_file(file_path).as_str())
}

// We solve this using dynamic programming again, similar to last
// question. In this one, we build the solution column by
// column. Getting to (i,j) could be from any of the (k,j-1) position
// on the previous column and then walking along the j-1-th column
// before getting to (i,j-1) and moving to the next column.
fn path_sum_three(text: &str) -> u32 {
    let grid = parse::parse_grid(text, ",");
    let (m, n) = (grid.len(), grid[0].len());
    let mut col_sums: Vec<u32> = (0..m).map(|i| grid[i][0]).collect();
    for j in 1..n {
	let mut new_col_sums = Vec::new();
	for i in 0..m {
	    let mut val = col_sums[i];
	    let mut sum = 0u32;
	    for k in (0..i).rev() {
		sum += grid[k][j];
		val = cmp::min(val, col_sums[k] + sum);
	    }
	    let mut sum = 0u32;
	    for k in (i+1)..m {
		sum += grid[k][j];
		val = cmp::min(val, col_sums[k] + sum);
	    }
	    new_col_sums.push(grid[i][j] + val);
	}
	col_sums = new_col_sums;
    }
    *col_sums.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_sum_three() {
	let text = "\
	131,673,234,103,18\n\
	201,96,342,965,150\n\
	630,803,746,422,111\n\
	537,699,497,121,956\n\
	805,732,524,37,331\
	";
	assert_eq!(path_sum_three(text), 994);
    }
}
