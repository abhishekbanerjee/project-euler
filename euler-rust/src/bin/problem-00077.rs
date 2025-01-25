use euler_rust::utils::primes::Primes;

fn main() {
    let num = prime_partition_over(5000);
    println!("{}", num);
}

// We are calculating the prime partition of numbers, using the Euler
// transform (described in
// https://math.stackexchange.com/a/89661). Since we use recursion
// over already calculated values, and the sum of prime factors of
// numbers, we cache these values.
fn prime_partition_over(threshold: u32) -> u32 {
    let mut primes = Primes::new();
    let mut prime_divisor_sums: Vec<u32> = vec![0, 0];
    let mut prime_partitions: Vec<u32> = vec![0,0];
    let mut n = 2u32;
    loop {
	let s = sum_of_prime_divisors(n as usize, &mut primes) as u32;
	prime_divisor_sums.push(s);
	let mut p = s;
	for i in 2..n {
	    p += prime_divisor_sums[i as usize] * prime_partitions[(n - i) as usize];
	}
	p /= n;
	prime_partitions.push(p);
	let count = if primes.is_prime(n as usize) { p - 1 } else { p };
	if count > threshold { break; }
	n += 1;
    }
    n
}

fn sum_of_prime_divisors(n: usize, prime_checker: &mut Primes) -> usize {
    if prime_checker.is_prime(n) { return n }
    let mut prime_idx = 0usize;
    let mut sum = 0usize;
    let mut m = n;
    loop {
	let p = prime_checker.prime_number(prime_idx);
	if p > m { break; }
	if m % p == 0 {
	    sum += p;
	    while m % p == 0 { m/= p; }
	}
	prime_idx += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_partition_over() {
	assert_eq!(prime_partition_over(4), 10);
    }
}
