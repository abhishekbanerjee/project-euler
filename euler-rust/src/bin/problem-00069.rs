use euler_rust::utils::primes::Primes;

fn main() {
    let max = totient_ratio_maximum(1_000_000);
    println!("{}", max);
}

// Per the totient formula, to get a larger n/phi(n) ratio (thus
// reduce the totient), we need a square-free product of primes. To
// pack these densely, we need a product of the first primes
// consecutively. These numbers are called primorial.
//
// Formally, we find the largest primorial number lower than the
// specified limit.
fn totient_ratio_maximum(limit: u32) -> u32 {
    let mut primes = Primes::up_to(limit as usize + 2);
    let mut n = 1u32;
    let mut primes_idx = 0usize;
    loop {
	let p = primes.prime_number(primes_idx) as u32;
	if n * p > limit { break; }
	n *= p;
	primes_idx += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_totient_ratio_maximum() {
	assert_eq!(totient_ratio_maximum(10), 6);
    }
}
