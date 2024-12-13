
// Sums up all the divisors of a number except for itself.
pub fn divisors_sum(n: u32) -> u32 {
    if n == 1 { return 0; }
    // The square root is the largest possible non-identity divisor of
    // a number.
    let root = (n as f64).sqrt().ceil() as u32;
    // Start the count at 1, because 1 always divides the
    // number.
    let mut divisors_sum = 1u32;
    for d in 2..root {
	// Each number d evenly dividing n gets two distinct divisors:
	// d and n/d. Since d < sqrt(n) by the loop condition,
	// therefore n/d > sqrt(n) and we therefore would not
	// encounter n/d in this loop.
	if n % d == 0 {
	    divisors_sum += d + n/d;
	}
    }
    // If n is an even square, the square root counts only once.
    if n == root*root {
	divisors_sum += root;
    }
    divisors_sum
}
