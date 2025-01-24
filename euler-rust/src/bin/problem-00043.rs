use euler_rust::utils::parse;
use euler_rust::utils::permutations::Permutations;
use euler_rust::utils::primes::Primes;

fn main() {
    let sum = substring_divisible(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    println!("{}", sum);
}

// For each permutation of the pandigital digits, we check the given
// conditions and sum up the matching ones.
fn substring_divisible(digits: &[u8]) -> usize {
    let mut permutations = Permutations::with(digits);
    let mut sum = 0usize;
    let prime_checks = first_primes(digits.len() - 3);
    loop {
	match permutations.next_permutation() {
	    None => break,
	    Some(state) => {
		if is_sub_divisible(&state, prime_checks.as_slice()) {
		    let n: usize = parse::parse_slice_as_number(&state);
		    sum += n;
		}
	    },
	}
    }
    sum
}

fn is_sub_divisible(digits: &[u8], prime_checks: &[u32]) -> bool {
    for i in 0..(digits.len() - 3) {
	let candidate: u32 = parse::parse_slice_as_number(&digits[(i+1)..(i+4)]);
	if candidate % prime_checks[i] != 0 {
	    return false
	}
    }
    true
}

// Find enough prime numbers to do our divisibility tests against.
fn first_primes(number: usize) -> Vec<u32> {
    let mut primes = Primes::new();
    let mut primes_vec = Vec::new();
    for idx in 0..number {
	primes_vec.push(primes.prime_number(idx) as u32);
    }
    primes_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sub_divisible() {
	assert!(is_sub_divisible(&[1,4,0,6,3,5,7,2,8,9], &[2,3,5,7,11,13,17]));
    }
}
