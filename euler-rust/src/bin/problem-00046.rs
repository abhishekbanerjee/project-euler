use euler_rust::utils::nums;
use euler_rust::utils::primes::Primes;

fn main() {
    let counter = goldbach_counter_example();
    println!("{}", counter);
}

fn goldbach_counter_example() -> usize {
    let mut primes = Primes::new();
    // Start with the first odd composite number.
    let mut candidate = 9usize;
    loop {
	// If the number itself is prime, we move on.
	if !primes.is_prime(candidate) {
	    let mut square_found = false;
	    // Check all odd primes below the number to see if they
	    // satisfy the conjecture.
	    for prime in primes.all_primes_below(candidate).iter() {
		if *prime == 2usize { continue; }
		if nums::is_perfect_square((candidate - *prime)/2) {
		    square_found = true;
		    break;
		}
	    }
	    // If no prime below the number satisfies, we found our
	    // counter-example!
	    if !square_found {
		return candidate
	    }
	}
	candidate += 2;
    }
}
