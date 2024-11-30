fn main() {
    let prime_factor = largest_prime_factor(600_851_475_143);
    println!("{}", prime_factor);
}

// We're using the Sieve of Eratosthenes method here to find primes.
fn largest_prime_factor(n: u64) -> u64 {
    // The square root of a number is the largest non-identity factor
    // possible, so we only need to check up until there.
    let root = (n as f64).sqrt().floor() as usize;
    let mut largest = 0u64;
    let mut sieve = vec![true; root+1];
    for factor in 2..=root {
	// If the number is already marked, it's composite.
	if !sieve[factor] {
	    continue;
	}
	// Otherwise mark multiples of the number in the sieve.
	let mut mark = factor*2;
	while mark <= root {
	    sieve[mark] = false;
	    mark += factor;
	}
	// If the prime divides the number in question, update the
	// largest found so far.
	if n % (factor as u64) == 0 {
	    largest = factor as u64;
	}
    }
    // This is true iff we didn't find a prime factor up to the square
    // root. This means the number itself is prime.
    if largest == 0 {
	return n
    }
    return largest
}
