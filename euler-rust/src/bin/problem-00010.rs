use euler_rust::utils::primes::Primes;

fn main() {
    let sum = sum_primes(2_000_000);
    println!("{}", sum);
}

// We use the Sieve of Eratosthenes method to find primes, and then
// sum them up.
fn sum_primes(n: usize) -> usize {
    Primes::up_to(n+1).all_primes_below(n).into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_primes() {
	assert_eq!(sum_primes(10), 17);
    }
}
