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
		let mut sub_divisible = true;
		for i in 0..(digits.len() - 3) {
		    let candidate: u32 = parse::parse_slice_as_number(&state[(i+1)..(i+4)]);
		    if candidate % prime_checks[i] != 0 {
			sub_divisible = false;
			break;
		    }
		}
		if sub_divisible {
		    let n: usize = parse::parse_slice_as_number(&state);
		    sum += n;
		}
	    },
	}
    }
    sum
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
