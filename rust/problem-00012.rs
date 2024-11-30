fn main() {
    let num = triangular_number_with_divisors(500);
    println!("{}", num);
}

fn triangular_number_with_divisors(d: u32) -> u64 {
    let mut seed = 2u64;
    // We run an infinite loop until we find a solution satisfying the
    // condition.
    loop {
	// We find the n-th triangular number by using the formmula
	// for the sum of the first n numbers.
	let num = if seed % 2 == 0 { seed/2*(seed+1) } else { (seed+1)/2*seed };
	let divisors = divisors_count(num);
	if divisors > d {
	    return num;
	}
	seed += 1;
    }
}

fn divisors_count(n: u64) -> u32 {
    // The square root is the largest possible non-identity divisor of
    // a number.
    let root = (n as f64).sqrt().floor() as u64;
    // Start the count at 2, for 1 and the number itself. [This
    // assumes that this function is not called for 1.]
    let mut divisors_count = 2u32;
    for d in 2..root {
	// Each number d evenly dividing n gets two distinct divisors:
	// d and n/d. Since d < sqrt(n) by the loop condition,
	// therefore n/d > sqrt(n) and we therefore would not
	// encounter n/d in this loop.
	if n % d == 0 {
	    divisors_count += 2;
	}
    }
    // If n is an even square, the square root counts as only one
    // divisor.
    if n % root == 0 {
	divisors_count += 1;
    }
    divisors_count
}
