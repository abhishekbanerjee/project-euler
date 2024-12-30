use euler_rust::utils::parse;
use std::collections::HashMap;

fn main() {
    let smallest = smallest_cubic_permutations(5);
    println!("{}", smallest);
}

// Map each cube to its canonical value. Count canonical value
// occurences. Return when we have the right amount.
fn smallest_cubic_permutations(permutations: u8) -> u64 {
    let mut canonical_count: HashMap<u64, u8> = HashMap::new();
    let mut canonical_smallest: HashMap<u64, u64> = HashMap::new();
    let mut current_digits = 0usize;
    let mut seed = 2u64;
    loop {
	let cube = seed * seed * seed;
	let l = cube.to_string().chars().count();
	// Whenever the number of digits in the cube changes, we do
	// our accounting. This is because we want the number with
	// exactly the right number of valid permutations, not at
	// least that number. When the number of digits changes, we
	// know that the canonical values we have collected so far
	// have exhausted their permutations.
	if l > current_digits {
	    // Collect all satisfying candidates (there could be more
	    // than one).
	    let mut candidates: Vec<u64> = Vec::new();
	    for (canonical, count) in &canonical_count {
		if *count == permutations {
		    candidates.push(*canonical_smallest.get(canonical).unwrap());
		}
	    }
	    if candidates.len() != 0 {
		return *candidates.iter().min().unwrap();
	    }
	    // Might as well reset the state since all current
	    // permutations are not useful.
	    canonical_count = HashMap::new();
	    canonical_smallest = HashMap::new();
	    current_digits = l;
	}
	let canonical = parse::canonicalize(cube);
	if canonical_count.contains_key(&canonical) {
	    let count = canonical_count[&canonical];
	    canonical_count.insert(canonical, count + 1);
	} else {
	    // If we're encountering this canonical value for the
	    // first time, the current cube is the smallest
	    // statisfying permutation, since cubes are monotonically
	    // increasing.
	    canonical_count.insert(canonical, 1);
	    canonical_smallest.insert(canonical, cube);
	}
	seed += 1;
    }
}
