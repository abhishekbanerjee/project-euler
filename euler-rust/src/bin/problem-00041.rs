use euler_rust::utils::parse;
use euler_rust::utils::permutations::Permutations;
use euler_rust::utils::primes::Primes;

fn main() {
    // We will consider only permutations of 7 digit pandigital
    // numbers. All 8 and 9 digit pandigital numbers are divisible by
    // 9. When I ran this code, I found a 7 digit pandigital prime, so
    // there's no need to go lower.
    let prime = largest_pandigital_prime(&[1, 2, 3, 4, 5, 6, 7]);
    println!("{}", prime);
}

fn largest_pandigital_prime(digits: &[u8]) -> usize {
    let mut primes = Primes::up_to(10usize.pow(digits.len() as u32 + 1));
    let mut permutations = Permutations::with(digits);
    let mut largest = 0usize;
    loop {
	match permutations.next_permutation() {
	    None => break,
	    Some(permutation) => {
		let candidate: usize = parse::parse_slice_as_number(permutation);
		// Since the permutations are in lexicographic order,
		// the next one is larger than all previous ones, so
		// we don't need a number comparison.
		if primes.is_prime(candidate) {
		    largest = candidate;
		}
	    }
	}
    }
    largest
}
