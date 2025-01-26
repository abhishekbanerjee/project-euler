use euler_rust::utils::nums;
use euler_rust::utils::primes::Primes;
use std::collections::HashSet;

fn main() {
    let count = prime_power_sums_below(50_000_000);
    println!("{}", count);
}

// Just run a triple loop over the primes making sure that the sum of
// the appropriate exponents of the appropriate primes does not flow
// over the supplied limit. Store the final remainders in a set (to
// prevent duplicates) and return the size of the set at the end.
fn prime_power_sums_below(threshold: usize) -> usize {
    let mut remainders: HashSet<usize> = HashSet::new();
    let mut primes = Primes::up_to(2 * nums::int_square_root(threshold));
    let mut idx1 = 0usize;
    loop {
	let p1 = primes.prime_number(idx1);
	if p1 * p1 > threshold { break; }
	let threshold2 = threshold - p1 * p1;
	let mut idx2 = 0usize;
	loop {
	    let p2 = primes.prime_number(idx2);
	    if p2 * p2 * p2 > threshold2 { break; }
	    let threshold3 = threshold2 - p2 * p2 * p2;
	    let mut idx3 = 0usize;
	    loop {
		let p3 = primes.prime_number(idx3);
		if p3 * p3 * p3 * p3 > threshold3 { break; }
		remainders.insert(threshold3 - p3 * p3 * p3 * p3);
		idx3 += 1;
	    }
	    idx2 += 1
	}
	idx1 += 1;
    }
    remainders.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_power_sums_below() {
	assert_eq!(prime_power_sums_below(50), 4);
    }
}
