use euler_rust::utils::parse;
use std::collections::HashMap;

fn main() {
    let count = factorial_chains_sixty(1_000_000);
    println!("{}", count);
}

// Traverse the factorial digit chains as outlined in the question,
// with a lot of caching.
fn factorial_chains_sixty(limit: u32) -> u32 {
    // Compute digit -> digit factorial first up and use it everywhere.
    let digit_factorials = compute_digit_factorials();
    // Cache of number -> factorial chain length.
    let mut chain_map: HashMap<u32, u8> = HashMap::new();
    let mut count = 0u32;
    for n in 1..limit {
	if chain_length(n, &mut chain_map, digit_factorials.as_slice()) == 60 { count += 1; }
    }
    count
}

// Compute and cache the chain length starting from the input number n.
fn chain_length(n: u32, chain_map: &mut HashMap<u32, u8>, digit_factorials: &[u32]) -> u8 {
    // Storing the chain starting at the number.
    let mut chain: Vec<u32> = Vec::new();
    // m tracks the number under consideration. Will be replaced by
    // the sum of the factorial of digits of itself.
    let mut m = n;
    let mut found_chain = false;
    loop {
	// There are two break conditions. The first is if the current
	// number is already in the chain. The found_chain boolean
	// tracks this. The second is if the current number is in our
	// cache.
	if chain.contains(&m) {
	    found_chain = true;
	    break;
	} else if chain_map.contains_key(&m) {
	    break;
	}
	// If none of the break conditions are satisfied, push the
	// current element in the chain, and overwrite it with the
	// next value to carry on.
	chain.push(m);
	m = parse::split_number_to_digits::<u32, usize>(m).into_iter().map(|d| digit_factorials[d]).sum();
    }
    // We now populate the cache for all elements encountered in the
    // chain, for either break condition.
    if found_chain {
	// In this case, the chain consists of an initial
	// non-repeating part, and then a periodic repeating part. We
	// have to account for the length of the non-repeating and
	// repeating parts separately.
	let mut l = chain.len() as u8;
	let mut encountered_cycle = false;
	for &p in chain.iter() {
	    chain_map.insert(p, l);
	    if p == m { encountered_cycle = true; }
	    if !encountered_cycle { l -= 1; }
	}
    } else { // Cached value encountered.
	// In this case, the length for each number in the chain is
	// calculated from it's position in the chain and the cached
	// value we enountered.
	let mut l = chain.len() as u8;
	for &p in chain.iter() {
	    chain_map.insert(p, l + chain_map[&m]);
	    l -= 1;
	}
    }
    chain_map[&n]
}

fn compute_digit_factorials() -> Vec<u32> {
    let mut factorials = vec![1u32];
    for n in 1u32..=9 {
	factorials.push(n * factorials[n as usize - 1]);
    }
    factorials
}
