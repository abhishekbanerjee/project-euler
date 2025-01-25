use euler_rust::utils::parse;
use euler_rust::utils::primes::Primes;

fn main() {
    let prime = prime_digit_replacements(8u8);
    println!("{}", prime);
}

fn prime_digit_replacements(limit: u8) -> usize {
    let mut primes = Primes::new();
    let mut idx = 0usize;
    // Loop over all primes in order.
    loop {
	let prime = primes.prime_number(idx);
	let l = prime.to_string().len();
	let digits_vec: Vec<u8> = parse::split_number_to_digits(prime);
	// If this were the first prime in the series, the digit to
	// replace would be in 0..=(10-limit).
	for digit in 0..=(10-limit) {
	    // If the number does not contain the digit to replace,
	    // continue.
	    if !digits_vec.contains(&digit) { continue; }
	    let mut count = 1u8;
	    // Loop over the replacement candidates for the digit.
	    for replacement in 0..=9 {
		// If the replacement is the same as the digit,
		// continue, since this is the original prime, which
		// we have already counted.
		if replacement == digit { continue; }
		// Build the number by replacing every occurence of
		// "digit" with "replacement".
		let candidate: usize = parse::parse_slice_as_number(
		    digits_vec.iter()
			.map(|d| if *d == digit { replacement } else { *d })
			.collect::<Vec<u8>>()
			.as_slice());
		// We need to check the length of the resulting number
		// to eliminate the leading zero case. Lastly, we
		// check if the resulting number is prime.
		if candidate.to_string().len() == l && primes.is_prime(candidate) {
		    count += 1;
		}
	    }
	    if count >= limit {
		return prime
	    }
	}
	idx += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consecutive_prime_sum() {
	assert_eq!(prime_digit_replacements(6), 13);
	assert_eq!(prime_digit_replacements(7), 56003);
    }
}
