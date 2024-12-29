use euler_rust::utils::parse;
use num_bigint::BigUint;

fn main() {
    let count = root_convergents_fractions(1000);
    println!("{}", count);
}

// We keep track of the numerator and denominator parts of the
// fractional part of the expression (the part after the 1 + ... in
// the expression). This part evoles formulaically: if the current
// fractional part is n/d, then the next fractional part is 1/(2 +
// n/d) = d/(2d + n).
fn root_convergents_fractions(iterations: u32) -> u32 {
    let mut numerator_part = BigUint::from(1u8);
    let mut denominator = BigUint::from(2u8);
    let mut count = 0u32;
    for _ in 0..iterations {
	// Remember to add the 1 back to the fraction before counting
	// digits, so the final numerator is the sum of the numerator
	// and denominator of the fractional part.
	if parse::count_digits(numerator_part.clone() + denominator.clone()) > parse::count_digits(denominator.clone()) {
	    count += 1;
	}
	let new_denominator = numerator_part + BigUint::from(2u8) * denominator.clone();
	numerator_part = denominator;
	denominator = new_denominator;
    }
    count
}
