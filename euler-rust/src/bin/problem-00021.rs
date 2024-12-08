fn main() {
    let sum = amicable_numbers_sum(10_000);
    println!("{}", sum);
}

fn amicable_numbers_sum(limit: u32) -> u64 {
    let mut amicable_sum = 0u64;
    for n in 2..limit {
	let d = divisors_sum(n);
	// We eliminate all numbers for which d(n) > n, to prevent
	// double counting. We also eliminate d(n) = n because
	// amicable pairs are distinct by definition.
	if d >= n {
	    continue;
	}
	if divisors_sum(d) == n {
	    amicable_sum += (d + n) as u64;
	}
    }
    amicable_sum
}

fn divisors_sum(n: u32) -> u32 {
    // The square root is the largest possible non-identity divisor of
    // a number.
    let root = (n as f64).sqrt().floor() as u32;
    // Start the count at 1, because 1 always divides the
    // number. [This assumes that this function is not called for 1.]
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
    if n % root == 0 {
	divisors_sum += root;
    }
    divisors_sum
}
