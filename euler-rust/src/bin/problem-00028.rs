fn main() {
    let sum = sum_spiral_diagonals(1001);
    println!("{}", sum);
}

// We will move out from the center, adding the elements at the
// diagonal of the spiral as we encounter them.
fn sum_spiral_diagonals(n: u32) -> u32 {
    // Initialize to 1, to count the number in the centre of the
    // spiral.
    let mut sum = 1u32;
    let mut current = 1u32;
    // The gap between adjacent diagonal numbers grows by 2 for each
    // step we move out.
    for step in (2..n).step_by(2) {
	// Four elements in the current diagonal.
	for _ in 0..4 {
	    current += step;
	    sum += current;
	}
    }
    sum
}
