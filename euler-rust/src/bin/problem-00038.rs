use std::collections::HashSet;
use euler_rust::utils::parse;

fn main() {
    let largest = largest_pandigital_multiple(vec![1u32, 2, 3, 4, 5, 6, 7, 8, 9]);
    println!("{}", largest);
}

fn largest_pandigital_multiple(digits: Vec<u32>) -> u64 {
    let digit_set: HashSet<u32> = digits.into_iter().collect();
    let l = digit_set.len();
    // Since we have to multiple the number by at least 2, we only
    // have to consider numbers up to half the length.
    let limit = 10u32.pow((l/2) as u32 + 1);
    let mut largest = 0u64;
    let mut largest_first = 0u32;
    for n in 2..limit {
	// Since the first part of a candidate pandigital number would
	// be the multiplicand itself, if its first digit is less than
	// the first digit of the largest pandigital number we've
	// found so far, we can discard this.
	if parse::first_digit(n) < largest_first {
	    continue;
	}
	let mut candidate_set: HashSet<u32> = HashSet::new();
	let mut candidate_vec: Vec<u64> = Vec::new();
	let mut failed = false;
	// Loop through the multipliers. Since we only get to 5 for
	// the smallest successful multiplicand: 9, this is a good
	// limit.
	for m in 1..=5 {
	    // Loop over the digits of the product.
	    for digit in parse::split_number_to_digits(n*m).iter() {
		// If the digit is not in our pandigital set, or we've
		// encountered it already, declare failure for this
		// multiplicand.
		if !digit_set.contains(&digit) || candidate_set.contains(&digit) {
		    failed = true;
		    break;
		}
		candidate_vec.push(digit.clone() as u64);
		candidate_set.insert(digit.clone());
	    }
	    if failed {
		break;
	    }
	    // We have a match!
	    if candidate_set.len() == l {
		let candidate : u64 = parse::parse_slice_as_number(candidate_vec.as_slice());
		// Check and update the largest, if needed.
		if candidate > largest {
		    largest = candidate;
		    largest_first = candidate_vec[0] as u32;
		}
		break;
	    }
	}
    }
    largest
}
