use std::collections::HashMap;

fn main() {
    let longest = longest_reciprocal_cycle(1000);
    println!("{}", longest);
}

fn longest_reciprocal_cycle(limit: u32) -> u32 {
    let mut longest = 1u32;
    let mut longest_length = 0u32;
    for n in 2..limit {
	let length = reciprocal_cycle_length(n);
	if length > longest_length {
	    longest_length = length;
	    longest = n;
	}
    }
    longest
}

// Just do long division, and keep track of all the remainders we've
// previously encountered. When we encounter an already encountered
// remainder, we have a cycle.
fn reciprocal_cycle_length(n: u32) -> u32 {
    let mut step = 0u32;
    let mut remainder = 1u32;
    let mut remainders = HashMap::new();
    remainders.insert(remainder, step);
    loop {
	step += 1;
	remainder = (remainder * 10) % n;
	// Even division. No cycle.
	if remainder == 0 { return 0; }
	match remainders.get(&remainder) {
	    // Match found! Let's return the difference as the cycle
	    // length.
	    Some(old_step) => return step - old_step,
	    // No match, so store this remainder with it's decimal
	    // place.
	    None => remainders.insert(remainder, step)
	};
    }
}
