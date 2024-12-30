use euler_rust::utils::primes::Primes;
use std::collections::HashMap;

fn main() {
    let sum = prime_pair_sets(5);
    println!("{}", sum);
}

// Check all sets of five primes (lowest first), until we find a
// match. The first match will have the lowest sum.
fn prime_pair_sets(set_cardinality: usize) -> usize {
    let mut primes = Primes::new();
    // For a pair of prime numbers (larger first), are their
    // concatenations both prime?
    let mut pair_status: HashMap<(usize, usize), bool> = HashMap::new();
    // Maps a set of prime indices (sorted in descending order) to if
    // the primes represented by them are all pairwise valid (both
    // concatenations prime).
    let mut set_status: HashMap<Vec<usize>, bool> = HashMap::new();
    // In each step, we will increase the largest prime index in the
    // set under consideration by 1.
    let mut current_idx = set_cardinality;
    loop {
	// Seed the sets under consideration with this prime.
	let mut current_set = vec![current_idx];
	match check(set_cardinality - 1,
		    &mut current_set,
		    &mut pair_status,
		    &mut set_status,
		    &mut primes) {
	    // We found a successful set!
	    Some(prime_set) => return prime_set
		.into_iter()
		.map(|e| primes.prime_number(e))
		.sum(),
	    None => (),
	}
	// Nothing found, continue the loop.
	current_idx += 1;
    }
}

// Recursively builds a set of indices to check the conditions. The
// invariant we will maintain is that when this method is called, all
// the elements in current_set already satisfy the condition.
fn check(elements_left: usize,
	 current_set: &mut Vec<usize>,
	 pair_status: &mut HashMap<(usize, usize), bool>,
	 set_status: &mut HashMap<Vec<usize>, bool>,
	 prime_checker: &mut Primes) -> Option<Vec<usize>> {
    // No more elements to add. If we got here, all elements in the
    // current_set are vetted, so return success.
    if elements_left == 0 {
	return Some(current_set.clone())
    }
    // Recurse backward over all indices below the last index in the
    // current_set.
    for n in (elements_left..*current_set.last().unwrap()).rev() {
	// Add element to the current_set.
	current_set.push(n);
	let current_set_copy = current_set.to_vec();
	// Test to see if this set is valid so far.
	let valid = match set_status.get(&current_set_copy) {
	    Some(result) => *result,
	    None => {
		let actual_result = test_set(current_set.as_slice(), pair_status, set_status, prime_checker);
		set_status.insert(current_set_copy, actual_result);
		actual_result
	    },
	};
	if valid {
	    // Now recursively attempt to add more elements to the
	    // (valid) set.
	    let result = check(elements_left - 1, current_set, pair_status, set_status, prime_checker);
	    // If the result is successful, report back.
	    if result.is_some() {
		return result
	    }
	}
	// Pop out the current element to restore current_set to its
	// state to continue exploring.
	current_set.pop();
    }
    None
}

// Checks if all the indices in the set are pairwise valid.
fn test_set(set: &[usize],
	    pair_status: &mut HashMap<(usize, usize), bool>,
	    set_status: &mut HashMap<Vec<usize>, bool>,
	    prime_checker: &mut Primes) -> bool {
    let l = set.len();
    // A single element set is always valid.
    if l == 1 { return true }
    // If the set is just a pair, just check the pair.
    if l == 2 {
	let p1 = prime_checker.prime_number(set[0]);
	let p2 = prime_checker.prime_number(set[1]);
	return *pair_status.entry((p1, p2)).or_insert_with(|| test_pair(p1, p2, prime_checker));
    }
    // Recursion. Split off the first part of the set from the last
    // element.
    let first_part = set[..l-1].to_vec();
    // First check the rest of the set.
    let mut valid = match set_status.get(&first_part) {
	Some(result) => *result,
	None => {
	    let actual_result = test_set(&first_part, pair_status, set_status, prime_checker);
	    set_status.insert(first_part, actual_result);
	    actual_result
	},
    };
    // Then pairwise check the last element against all previous
    // elements in the set.
    let p2 = prime_checker.prime_number(set[l-1]);
    for i in 0..l-1 {
	let p1 = prime_checker.prime_number(set[i]);
	valid &= *pair_status.entry((p1, p2)).or_insert_with(|| test_pair(p1, p2, prime_checker));
    }
    valid
}

// Given a pair of primes, check if their concatenations (both ways)
// are prime or not.
fn test_pair(p1: usize, p2: usize, prime_checker: &mut Primes) -> bool {
    prime_checker.is_prime_fast(concatenate(p1, p2)) && prime_checker.is_prime_fast(concatenate(p2, p1))
}

// Concatenate two numbers: convert both numbers to string,
// concatenate strings, and convert back concatenated string to
// number.
fn concatenate(n1: usize, n2: usize) -> usize {
    let mut s = n1.to_string();
    s.push_str(&n2.to_string());
    s.parse::<usize>().unwrap()
}
