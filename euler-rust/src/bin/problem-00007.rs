use euler_rust::utils::primes;

fn main() {
    let prime = find_prime_number(10_001);
    println!("{}", prime);
}

// We use the Sieve of Eratosthenes method to find primes until we
// find the n-th one.
fn find_prime_number(n: usize) -> usize {
    let mut primes = primes::Primes::new();
    // The index of the n-th prime number would be n-1.
    primes.prime_number(n-1)
}
