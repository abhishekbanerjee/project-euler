use euler_rust::utils::parse;
use num_bigint::BigUint;
use std::cmp;

fn main() {
    let sum = max_digital_sum(100, 100);
    println!("{}", sum);
}

fn max_digital_sum(base_limit: u8, exponent_limit: u8) -> u32 {
    let mut max = 0u32;
    for base_small in 2..base_limit {
	let mut product = BigUint::from(1u32);
	let base = BigUint::from(base_small);
	for _ in 1..exponent_limit {
	    // At each step, multiply the product by the base to
	    // increase the exponent by 1.
	    product *= base.clone();
	    // Split into digits and sum them up.
	    let digital_sum: u32 = parse::split_number_to_digits::<BigUint, u32>(product.clone()).into_iter().sum();
	    max = cmp::max(max, digital_sum);
	}
    }
    max
}
