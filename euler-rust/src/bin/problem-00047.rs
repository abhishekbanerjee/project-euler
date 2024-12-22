use euler_rust::utils::primes::Primes;
use std::collections::HashSet;

fn main() {
    let first = distinct_prime_factors(4);
    println!("{}", first);
}

// We loop over the numbers. For each number, we find it's prime
// factors, check if the number of prime factors is the right amount
// and if they're disjoint with the previous number's prime factors
// (if present). Once we find a series of the right length, we return.
fn distinct_prime_factors(consecutive: u8) -> usize {
    let mut previous_factors: HashSet<usize> = HashSet::new();
    let mut candidate = 2usize;
    let mut found = 0u8;
    let mut primes = Primes::new();
    loop {
	let factors = primes.prime_factors(candidate);
	if factors.len() == consecutive as usize {
	    if found > 0 && factors.is_disjoint(&previous_factors) {
		found += 1;
	    } else {
		found = 1;
	    }
	    if found == consecutive {
		return candidate - (consecutive as usize) + 1
	    }
	    previous_factors = factors;
	} else {
	    found = 0;
	    previous_factors = HashSet::new();
	}
	candidate += 1;
    }
}
