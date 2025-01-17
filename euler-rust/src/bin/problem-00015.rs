fn main() {
    let paths = lattice_paths(20);
    println!("{}", paths);
}

// We use dynamic programming for this. To get to point (i,j) in the
// grid, you can get there through (i-1,j) or (i, j-1). So the number
// of paths at that point is simply the sum of the number of paths in
// the other two points. To simplify this a little further, we observe
// that populating the current row only needs the previous row and
// none of the previous ones, so that is all we work with. Last, the
// text of the question talks about the corners in an n x n grid,
// which translates to positions in a (n+1) x (n+1) grid.
fn lattice_paths(n: usize) -> u64 {
    let mut current_row = vec![1u64; n+1];
    for _ in 0..n {
	let mut new_row = vec![1u64];
	for j in 1..=n {
	    new_row.push(current_row[j] + new_row[j-1]);
	}
	current_row = new_row;
    }
    current_row[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lattice_paths() {
	assert_eq!(lattice_paths(2), 6);
    }
}
