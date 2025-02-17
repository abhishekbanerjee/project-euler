use euler_rust::utils::files;
use euler_rust::utils::parse;
use std::cmp;
use std::collections::HashMap;

fn main() {
    let sum = path_sum_four_file("resources/0083_matrix.txt");
    println!("{}", sum);
}

fn path_sum_four_file(file_path: &str) -> u32 {
    path_sum_four(files::read_file(file_path).as_str())
}

// We solve this differently from the previous parts. In this one, we
// model the grid as graph, with each position being a vertex and
// adjacent positions being neighbours in the graph. The problem now
// reduces to finding the shortest path in this graph, which we solve
// using Dijkstra's algorithm.
fn path_sum_four(text: &str) -> u32 {
    let grid = parse::parse_grid(text, ",");
    let (m, n) = (grid.len(), grid[0].len());
    let mut distances = HashMap::from([((0usize, 0usize), grid[0][0])]);
    for i in 0..m {
	for j in 0..n {
	    if i != 0 || j != 0 { distances.insert((i, j), u32::MAX); }
	}
    }
    loop {
	let (&(i, j), &d) = distances.iter().min_by_key(|&(_, v)| v).unwrap();
	distances.remove(&(i,j));
	if i == m-1 && j == n-1 { return d }
	if i != 0 && distances.contains_key(&(i-1, j)) { distances.insert((i-1, j), cmp::min(distances[&(i-1, j)], d + grid[i-1][j])); }
	if j != 0 && distances.contains_key(&(i, j-1)) { distances.insert((i, j-1), cmp::min(distances[&(i, j-1)], d + grid[i][j-1])); }
	if i != m-1 && distances.contains_key(&(i+1, j)) { distances.insert((i+1, j), cmp::min(distances[&(i+1, j)], d + grid[i+1][j])); }
	if j != n-1 && distances.contains_key(&(i, j+1)) { distances.insert((i, j+1), cmp::min(distances[&(i, j+1)], d + grid[i][j+1])); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_sum_four() {
	let text = "\
	131,673,234,103,18\n\
	201,96,342,965,150\n\
	630,803,746,422,111\n\
	537,699,497,121,956\n\
	805,732,524,37,331\
	";
	assert_eq!(path_sum_four(text), 2297);
    }
}
