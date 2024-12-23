use euler_rust::utils::parse;
use euler_rust::utils::primes::Primes;
use num_bigint::BigUint;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let seq = prime_permutations_sequence(BigUint::from(148748178147u128));
    println!("{}", seq);
}

fn prime_permutations_sequence(exception: BigUint) -> BigUint {
    let u_limit = 10_000usize;
    let l_limit = 1_000usize;
    let mut primes = Primes::up_to(u_limit);
    let mut primes_idx = 0usize;
    while primes.prime_number(primes_idx) < l_limit { primes_idx += 1; }
    let mut canonical_primes = HashMap::new();
    // Sort all 4-digit primes into buckets by their canonicalized
    // forms. Primes falling in the same bucket are permutations of
    // each other.
    loop {
	let prime = primes.prime_number(primes_idx);
	if prime >= u_limit { break; }
	let canonical = parse::canonicalize(prime);
	if canonical >= l_limit {
	    if !canonical_primes.contains_key(&canonical) {
		canonical_primes.insert(canonical, Vec::new());
	    }
	    canonical_primes.get_mut(&canonical).unwrap().push(prime);
	}
	primes_idx += 1;
    }
    // Loop over the canonical buckets.
    for permutations in canonical_primes.values_mut() {
	// If there are less than 3 prime permutations, discard.
	if permutations.len() < 3 { continue; }
	// Create a hashset to cheaply check for membership later.
	let perm_set: HashSet<usize> = permutations.clone().into_iter().collect();
	// Sort the primes, so we can consider the arithmetic sequence
	// in order.
	permutations.sort();
	// Do a quadratic loop over the primes to obtain the first two
	// primes in the sequence.
	for i in 0..permutations.len() {
	    let first = permutations[i];
	    for j in (i+1)..permutations.len() {
		let second = permutations[j];
		// Take the third term in this sequence.
		let third = second + (second - first);
		// If this term is also a prime permutation, we have
		// satisfied all requirements.
		if perm_set.contains(&third) {
		    let num: BigUint = parse::parse_slice_as_number(&[first, second, third]);
		    // Make sure we're not returning the example from
		    // the question.
		    if num != exception {
			return num
		    }
		}
	    }
	}
    }
    // The question guarantees that we will have another sequence, so
    // we will not reach here.
    panic!("Impossible!");
}
