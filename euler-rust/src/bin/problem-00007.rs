fn main() {
    let prime = find_prime_number(10_001);
    println!("{}", prime);
}

// We use the Sieve of Eratosthenes method to find primes until we
// find the n-th one. There is some added complexity for the case
// where we don't know how big the sieve needs to be. We do this by
// doubling the sieve. We also have to store all the primes discovered
// so far to mark their multiples off when we extend the sieve.
fn find_prime_number(n: u32) -> u64 {
    let mut sieve = vec![true; 100_000];
    let mut primes: Vec<u64> = Vec::new();
    let mut current_factor: u64 = 2;
    while (primes.len() as u32) < n {
	// This is the part where the sieve is smaller than we needed,
	// so we need to make it bigger.
	if current_factor >= sieve.len() as u64 {
	    let current_length = sieve.len();
	    double(&mut sieve);
	    // Mark off the multiples of currently discovered primes
	    // in the extended portion of the sieve.
	    for prime in primes.iter() {
		mark(sieve.as_mut_slice(), *prime, current_length as u64);
	    }
	}
	if sieve[current_factor as usize] {
	    primes.push(current_factor);
	    // Mark off all multiples of the newly discovered prime in
	    // the sieve.
	    mark(sieve.as_mut_slice(), current_factor, current_factor + 1);
	}
	current_factor += 1;
    }
    return *primes.last().expect("No primes found!");
}

fn double(sieve: &mut Vec<bool>) {
    let length = sieve.len();
    sieve.extend(vec![true; length]);
}

fn mark(sieve: &mut [bool], prime: u64, start_at: u64) {
    let mut idx = (((start_at as f64)/(prime as f64)).ceil() as u64) * prime;
    while idx < (sieve.len() as u64) {
	sieve[idx as usize] = false;
	idx += prime;
    }
}
