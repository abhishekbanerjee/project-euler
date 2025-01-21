use num_bigint::BigUint;

fn main() {
    let sum = fibonacci_term_with_digits(1000);
    println!("{}", sum);
}

fn fibonacci_term_with_digits(digits: usize) -> u32 {
    let mut a = BigUint::from(1u32);
    let mut b = BigUint::from(1u32);
    let mut term_number = 2;
    loop {
	term_number += 1;
	// Keep the last 2 Fibonacci numbers around, and use them to
	// compute the next one.
	let c = a + b.clone();
	a = b;
	b = c;

	// Check if we have arrived at the correct number of digits.
	if b.to_str_radix(10).len() >= digits { break; }
    }
    return term_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_term_with_digits() {
	assert_eq!(fibonacci_term_with_digits(3), 12);
    }
}
