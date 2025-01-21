use euler_rust::utils::primes::Primes;
use std::collections::HashSet;

fn main() {
    let count = circular_primes(1_000_000);
    println!("{}", count);
}

fn circular_primes(limit: usize) -> usize {
    let mut circular_primes = HashSet::new();
    let mut primes = Primes::up_to(limit+1);
    for prime in primes.all_primes_below(limit).into_iter() {
	if circular_primes.contains(&prime) { continue; }
	let mut is_circular_prime = true;
	let prime_rotations = rotations(prime);
	// We can skip the first number in the rotations, since it is
	// the number itself.
	for &rotated in prime_rotations.iter().skip(1) {
	    if !primes.is_prime(rotated) {
		is_circular_prime = false;
		break;
	    }
	}
	// If this is a circular prime, store all rotations for it.
	if is_circular_prime {
	    circular_primes.extend(prime_rotations.into_iter());
	}
    }
    circular_primes.len()
}

// Get all the rotated numbers for a given number.
fn rotations(n: usize) -> Vec<usize> {
    let mut m = n.clone();
    let l = n.to_string().len();
    let mut rotated = vec![n];
    for _ in 1..l {
	m = 10usize.pow((l-1) as u32) * (m%10) + m/10;
	rotated.push(m);
    }
    rotated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_primes() {
	assert_eq!(circular_primes(100), 13);
    }
}
