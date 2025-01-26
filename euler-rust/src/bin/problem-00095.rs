use euler_rust::utils::nums;
use std::collections::HashSet;

fn main() {
    let longest = longest_amicable_chain(1_000_000);
    println!("{}", longest);
}

// Start forming chains at each number and check the length of the
// resulting chain. Return the longest.
fn longest_amicable_chain(limit: u32) -> u32 {
    let mut longest_chain_length = 0usize;
    let mut longest_chain_representative = 0u32;
    // Keeps track of the numbers we have encountered so far.
    let mut encountered: HashSet<u32> = HashSet::from([0]);
    for n in 1..=limit {
	// If we've seen this number already, carry on.
	if encountered.contains(&n) { continue; }
	let mut chain: Vec<u32> = Vec::new();
	let mut m = n;
	// We stop if we encounter a number we've seen or if we exceed
	// the limit.
	while !chain.contains(&m) && m <= limit {
	    chain.push(m);
	    m = nums::divisors_sum(m);
	}
	if chain.contains(&m) {
	    // Find the starting point of the repeaating part.
	    let mut idx = 0usize;
	    while chain[idx] != m { idx += 1; }
	    // Compare the length of the repeating part with the
	    // longest found so far.
	    if chain.len() - idx > longest_chain_length {
		longest_chain_length = chain.len() - idx;
		longest_chain_representative = *chain.iter().skip(idx).min().unwrap();
	    }
	}
	// Add all numbers in the current iteration to the set of
	// numbers we've seen.
	encountered.extend(chain.into_iter());
    }
    longest_chain_representative
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_amicable_chain() {
	// The chain in the question is the longest chain below
	// 100,000.
	assert_eq!(longest_amicable_chain(100_000), 12496);
    }
}
