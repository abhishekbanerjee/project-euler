use euler_rust::utils::primes::Primes;

fn main() {
    let max_primes_coefficients = max_quadratic_primes(1000, 1000);
    println!("{}", max_primes_coefficients);
}

fn max_quadratic_primes(a_limit: i32, b_limit: i32) -> i32 {
    let mut max_primes = 0usize;
    let mut max_product = 0i32;
    let mut prime_checker = Primes::new();
    // Loop through candidate values of "a" and "b" within the
    // specified limits, and check prime counts for each pair.
    for a in (-1*a_limit + 1)..a_limit {
	for b in (-1*b_limit)..=b_limit {
	    let primes = primes_count(&mut prime_checker, a as i64, b as i64);
	    if primes > max_primes {
		max_primes = primes;
		max_product = a*b;
	    }
	}
    }
    max_product
}

// Loop through n, check each value of the formula to see if it is
// prime, stop when composite.
fn primes_count(prime_checker: &mut Primes, a: i64, b: i64) -> usize {
    let mut n = 0i64;
    loop {
	let candidate = n*n + a*n + b;
	if prime_checker.is_prime_fast(candidate.unsigned_abs() as usize) {
	    n += 1;
	} else {
	    break;
	}
    }
    n.unsigned_abs() as usize
}
