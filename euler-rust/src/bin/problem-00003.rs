use euler_rust::utils::nums;
use euler_rust::utils::primes;

fn main() {
    let prime_factor = largest_prime_factor(600_851_475_143);
    println!("{}", prime_factor);
}

// We're using the Sieve of Eratosthenes method here to find primes.
fn largest_prime_factor(n: u64) -> u64 {
    // The square root of a number is the largest non-identity factor
    // possible, so we only need to check up until there.
    let root = nums::int_square_root(n) as usize;
    let mut largest = n;
    let mut primes = primes::Primes::up_to(root+1);
    let mut prime_idx = 0usize;
    loop {
	let prime = primes.prime_number(prime_idx);
	if prime > root as usize {
	    break;
	}
	if n % (prime as u64) == 0 {
	    largest = prime as u64;
	}
	prime_idx += 1;
    }
    largest
}
