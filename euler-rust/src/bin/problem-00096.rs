use euler_rust::utils::files;
use euler_rust::utils::parse;

fn main() {
    let sum = sudoku_solve_sum("resources/p096_sudoku.txt");
    println!("{}", sum);
}

// Parse the sudoku boards. For each board, solve the sudoku puzzle
// and sum up the first three numbers in the first row of the solved
// grids.
fn sudoku_solve_sum(file_name: &str) -> u32 {
    let mut board: Vec<Vec<u8>> = Vec::new();
    let mut solved_sums = 0u32;
    for (i, line) in files::read_file(file_name).lines().enumerate() {
	// First line announces a new board, ignore.
	if i % 10 == 0 { continue; }
	// Parse the current line as a row of the board..
	board.push(line.chars()
		   .into_iter()
		   .map(|c| c.to_string().parse::<u8>().unwrap())
		   .collect());
	// Once all the rows in the current board have been collected.
	if i % 10 == 9 {
	    // ... solve the board and add it to the solution, and
	    solved_sums += sudoku_solve(&mut board);
	    // ... clear out the board for the next puzzle.
	    board = Vec::new();
	}
    }
    solved_sums
}

fn sudoku_solve(board: &mut Vec<Vec<u8>>) -> u32 {
    let mut rows: Vec<u32> = vec![0; 9];
    let mut cols: Vec<u32> = vec![0; 9];
    let mut boxes: Vec<u32> = vec![0; 9];
    // Create the auxiliary info needed to quickly check the current
    // entry in the board.
    create_checkers(&*board, &mut rows, &mut cols, &mut boxes);
    // Actually solve the board via backtracking.
    sudoku_solve_recursive(board, &mut rows, &mut cols, &mut boxes);
    // Take the first three numbers from the first row of the solved
    // board.
    parse::parse_slice_as_number(&[board[0][0], board[0][1], board[0][2]])
}

fn sudoku_solve_recursive(board: &mut Vec<Vec<u8>>, rows: &mut Vec<u32>, cols: &mut Vec<u32>, boxes: &mut Vec<u32>) -> bool {
    // Find the next unfilled position on the board.
    let mut found = false;
    let mut i = 0usize;
    let mut j = 0usize;
    while i < 9 {
	j = 0;
	while j < 9 {
	    if board[i][j] == 0 {
		found = true;
		break;
	    }
	    j += 1;
	}
	if found == true { break; }
	i += 1;
    }
    // If we weren't able to find an unfilled position, the board is
    // solved!
    if !found { return true }
    // Otherwise, try to fill every number in the unfilled position.
    for n in 1u8..=9 {
	// Only fill in the number if it still leads to a valid board.
	if !check(n, i, j, rows.as_slice(), cols.as_slice(), boxes.as_slice()) { continue; }
	let n_mask = 1u32 << n;
	// Update the board and the auxiliary checkers.
	board[i][j] = n;
	rows[j] |= n_mask;
	cols[i] |= n_mask;
	boxes[box_number(i, j)] |= n_mask;
	// Recursive call to fill the rest of the board. If it works,
	// report success.
	if sudoku_solve_recursive(board, rows, cols, boxes) { return true }
	// Reset the current position in the board and the checkers so
	// that we can try the next value.
	board[i][j] = 0;
	rows[j] &= !n_mask;
	cols[i] &= !n_mask;
	boxes[box_number(i, j)] &= !n_mask;
    }
    // None of the values worked. Report failure.
    false
}

// Use the auxiliary checkers to see if adding n at (i,j) on the board
// would still result in a valid board.
fn check(n: u8, i: usize, j: usize, rows: &[u32], cols: &[u32], boxes: &[u32]) -> bool {
    let num_mask = 1u32 << n;
    rows[j] & num_mask == 0 && cols[i] & num_mask == 0 && boxes[box_number(i, j)] & num_mask == 0
}

// Using the current state of the board, fill in vectors with encoded
// state of each row, column and 3x3 box in the Sudoku grid. Each
// entity (row/col/box) is represented by an integer. If n is present
// in the entity, then the n-th bit of the integer would be set.
fn create_checkers(board: &Vec<Vec<u8>>, rows: &mut Vec<u32>, cols: &mut Vec<u32>, boxes: &mut Vec<u32>) {
    for i in 0..9 {
	for j in 0..9 {
	    if board[i][j] == 0 { continue; }
	    let num_mask = 1u32 << board[i][j];
	    rows[j] |= num_mask;
	    cols[i] |= num_mask;
	    boxes[box_number(i, j)] |= num_mask;
	}
    }
}

// Encoding for the 3x3 box where (i, j) is.
fn box_number(i: usize, j: usize) -> usize {
    i / 3 * 3 + j / 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku_solve() {
	let mut board: Vec<Vec<u8>> = vec![
	    vec![0,0,3,0,2,0,6,0,0],
	    vec![9,0,0,3,0,5,0,0,1],
	    vec![0,0,1,8,0,6,4,0,0],
	    vec![0,0,8,1,0,2,9,0,0],
	    vec![7,0,0,0,0,0,0,0,8],
	    vec![0,0,6,7,0,8,2,0,0],
	    vec![0,0,2,6,0,9,5,0,0],
	    vec![8,0,0,2,0,3,0,0,9],
	    vec![0,0,5,0,1,0,3,0,0]
	];
	assert_eq!(sudoku_solve(&mut board), 483);
    }
}
