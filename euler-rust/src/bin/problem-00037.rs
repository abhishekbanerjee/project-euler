use euler_rust::utils::primes::Primes;

fn main() {
    let sum = sum_truncatable_primes(11);
    println!("{}", sum);
}

fn sum_truncatable_primes(mut count: u8) -> usize {
    let mut sum = 0usize;
    let mut prime_idx = 0usize;
    let mut primes = Primes::new();
    while count != 0u8 {
	let prime = primes.prime_number(prime_idx);
	if is_truncatable_prime(prime, &mut primes) {
	    count -= 1;
	    sum += prime;
	}
	prime_idx += 1;   
    }
    sum
}

fn is_truncatable_prime(prime: usize, primes: &mut Primes) -> bool {
    let l = prime.to_string().len();
    if l == 1 { return false }
    // Div and mod by powers of 10 to get the rigt and left
    // truncations respectively. Check primality each step of the
    // way.
    for i in 1..l {
	let ten_pow = 10usize.pow(i as u32);
	if !primes.is_prime(prime % ten_pow) || !primes.is_prime(prime / ten_pow) { return false }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_truncatable_prime() {
	assert!(is_truncatable_prime(3797, &mut Primes::new()));
    }
}
