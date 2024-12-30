fn main() {
    let count = odd_period_square_root(10_000);
    println!("{}", count);
}

fn odd_period_square_root(limit: u32) -> u32 {
    // Test each number up to the limit and count successes.
    (1..=limit).map(|n| is_odd_period_square_root(n) as u32).sum()
}

// Use the algorithm outlined in the question to keep track of the
// important coefficients. For a given N, in each step we obtain a
// whole part, a, and a fraction (sqrt(N) - y)/x. We keep track of
// these (x, y) pairs, not keeping track of the whole parts. When we
// encounter the first pair again, we have completed our cycle.
fn is_odd_period_square_root(n: u32) -> bool {
    // Integer square root of the number.
    let r = (n as f32).sqrt().floor() as u32;
    // If the number is a perfect square, it is not periodic at all,
    // and therefore also not odd periodic.
    if r * r == n { return false }
    // Initialize our first pair of numbers to track.
    let mut pair: (u32, u32) = (1, r);
    let mut period = 0usize;
    loop {
	period += 1;
	let (old_x, old_y) = pair;
	let new_x = (n - old_y*old_y) / old_x;
	let a = (r + old_y) / new_x;
	let new_y = a * new_x - old_y;
	// Check if the new coefficients are the same as the first
	// ones.
	if new_x == 1 && new_y == r { break; }
	pair = (new_x, new_y);
    }
    period % 2 == 1
}
