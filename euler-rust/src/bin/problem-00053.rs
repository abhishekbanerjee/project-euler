use num_bigint::BigUint;

fn main() {
    let values = combinatoric_selections(100, BigUint::from(1_000_000u32));
    println!("{}", values);
}

fn combinatoric_selections(bound: u64, lower_limit: BigUint) -> usize {
    let mut values = 0usize;
    for n in 2..=bound {
	// Start counting from the mid-point outward, because the
	// mid-point is where the largest combinations are.
	let mut r = n/2;
	let mut c = combination(n, r);
	while r > 0 {
	    // All subsequent combinations will be smaller, so break
	    // here.
	    if c <= lower_limit { break; }
	    // Use the fact that combination(n, r) = combination(n,
	    // n-r). But make sure not to double-count the exact
	    // center!
	    values += if 2*r == n { 1 } else { 2 };
	    c = c * r / (n - r + 1);
	    r -= 1;
	}
    }
    values
}

fn combination(n: u64, m: u64) -> BigUint {
    if m == 0 { return BigUint::from(1u8) }
    let mut numerator = BigUint::from(n);
    let mut denominator = BigUint::from(1u8);
    for i in 2..=m {
	numerator *= n + 1 - i;
	denominator *= i;
    }
    numerator / denominator
}
