use std::collections::HashSet;
use euler_rust::utils::primes::Primes;

fn main() {
    let terms = distinct_powers(100, 100);
    println!("{}", terms);
}

// We keep a set of all the distinct prime factorizations of the
// numbers. We first factorize the base as exponents of primes, and
// then multiply each exponent by the exponent on the number. The
// total number of members of the set is our answer.
fn distinct_powers(a_limit: u32, b_limit: u32) -> usize {
    let mut terms = HashSet::new();
    let mut primes = Primes::up_to((a_limit+1) as usize);
    for a in 2..=a_limit {
	// Factorize the base first.
	let factors = prime_factorization(&mut primes, a.clone());
	for b in 2..=b_limit {
	    terms.insert(multiply(factors.as_slice(), b));
	}
    }
    terms.len()
}

fn prime_factorization(primes: &mut Primes, mut n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut prime_idx = 0usize;
    while n != 1 {
	let mut current_pow = 0u32;
	let current_prime = primes.prime_number(prime_idx) as u32;
	while n % current_prime == 0 {
	    n /= current_prime;
	    current_pow += 1;
	}
	// The representation we're choosing is a vector of
	// exponents. factors[i] would contain the power of the i-th
	// prime (0-based). The vector only goes as long as the
	// largest prime in the factorization. This is a good enough
	// representation for uniqueness, which is what we want here.
	factors.push(current_pow);
	prime_idx += 1;
    }
    factors
}

fn multiply(factorization: &[u32], multiplicand: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    for power in factorization.iter() {
	factors.push(*power * multiplicand);
    }
    factors
}
