use euler_rust::utils::parse;
use num_bigint::BigUint;

fn main() {
    let sum = numerator_sum_e_convergent(100);
    println!("{}", sum);
}

// Work backwards. Form the last fraction first, and keep working up
// until we get to the first one.
fn numerator_sum_e_convergent(limit: u8) -> u32 {
    let (mut n, mut d) = (BigUint::from(1u8), BigUint::ZERO);
    for i in (1..=limit).rev() {
	// The terms are on a cycle of 3. For every term a_t
	// (1-indexed) where t=3*u, a_t = 2*u. Otherwise a_t = 1. The
	// only exception is the first term, which is 2 and not 1.
	let t = BigUint::from(if i % 3 == 0 { 2 * i / 3 } else { if i == 1 { 2 } else { 1 } });
	let new_n = t * &n + d;
	d = n;
	n = new_n;
    }
    parse::split_number_to_digits::<BigUint, u32>(n).into_iter().sum()
}
