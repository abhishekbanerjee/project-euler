use euler_rust::utils::permutations::Permutations;
use std::collections::HashMap;

fn main() {
    let sum = cyclical_figurate_numbers(&[triangle, square, pentagonal, hexagonal, heptagonal, octagonal]);
    println!("{}", sum);
}

// Accepts a list of functions we want the cycles to satisfy.
fn cyclical_figurate_numbers(functions: &[fn(u32) -> u32]) -> u32 {
    let maps: Vec<HashMap<u32, Vec<u32>>> = functions.into_iter().map(|&f| function_to_map(f)).collect();
    // Since the members of the cycle could satisfy the functions in
    // any order, we need to permute the order of the functions to
    // cover all possibilities.
    let mut permutations = Permutations::with((0..maps.len()).collect::<Vec<usize>>().as_slice());
    loop {
	match permutations.next_permutation() {
	    None => break,
	    Some(idx_order) => {
		let mut empty: Vec<u32> = Vec::new();
		match find_cycle(&mut empty, idx_order, &maps) {
		    None => (),
		    // Cycle found! Build up the numbers in the cycle
		    // (the cycle itself only contains two digit
		    // prefix/suffix values), and sum them up.
		    Some(cycle) => {
			let last_value = join(*cycle.last().unwrap(), cycle[0]);
			return cycle.iter().zip(cycle.iter().skip(1)).map(|(&a, &b)| join(a, b)).sum::<u32>() + last_value
		    },
		}
	    },
	}
    }
    // The question guarantees that we will find an answer, so we will
    // not reach here.
    panic!("Impossible!")
}

// Recursively builds the cycle. The idx_order is a permutation of the
// indices of the functions represented by the maps. In each step, we
// will choose the next map based on the next index in idx_order. The
// invariant we will maintain is that when this method is called, all
// l elements in state are chosen in accordance with the relationships
// specified by the first (in the permuted order) l-1 maps.
fn find_cycle(state: &mut Vec<u32>, idx_order: &[usize], maps: &Vec<HashMap<u32, Vec<u32>>>) -> Option<Vec<u32>> {
    let l = state.len();
    if l == idx_order.len() {
	// At the end, we need to make sure that the last element and
	// the first element of the state we've built so far satisfy
	// the relationship in the last (in the permuted order) map.
	return if maps[idx_order[l-1]].get(&state[l-1]).map(|v| v.contains(&state[0])).unwrap_or(false) { Some(state.clone()) } else { None }
    }
    if l == 0 {
	// If the state does not have any members, we just start
	// populating it with the keys of the first (in the permuted
	// order) map.
	for &n in maps[idx_order[0]].keys() {
	    state.push(n);
	    let result = find_cycle(state, idx_order, maps);
	    if result.is_some() {
		return result
	    }
	    state.pop();
	}
	return None
    }
    // Otherwise, we build the next element in the state from the
    // mapping of the current last element in the state in the current
    // (in the permuted order) map.
    match maps[idx_order[l-1]].get(&state[l-1]) {
	None => return None,
	Some(v) => {
	    for &n in v.iter() {
		state.push(n);
		let result = find_cycle(state, idx_order, maps);
		if result.is_some() {
		    return result
		}
		state.pop();
	    }
	    return None
	}
    }
}

// Transforms the function into four digit values. Splits the four
// digit values into 2-digit high order digit keys -> 2-digit low
// order digit values. Since the same high order digits could map to
// multiple low order values, the output map is one-many.
fn function_to_map(function: fn(u32) -> u32) -> HashMap<u32, Vec<u32>> {
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut seed = 1u32;
    loop {
	let n = function(seed);
	seed += 1;
	if n > 9999 { break; }
	if n < 1000 { continue; }
	let value = map.entry(n/100).or_insert(Vec::new());
	value.push(n%100);
    }
    map
}

fn join(a: u32, b: u32) -> u32 {
    a * 100 + b
}

fn triangle(n: u32) -> u32 { n * (n + 1) / 2 }
fn square(n: u32) -> u32 { n * n }
fn pentagonal(n: u32) -> u32 { n * (3 * n - 1) / 2 }
fn hexagonal(n: u32) -> u32 { n * (2 * n - 1) }
fn heptagonal(n: u32) -> u32 { n * (5 * n - 3) / 2 }
fn octagonal(n: u32) -> u32 { n * (3 * n - 2) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cyclical_figurate_numbers() {
	assert_eq!(cyclical_figurate_numbers(&[triangle, square, pentagonal]), 8128 + 2882 + 8281);
    }
}
