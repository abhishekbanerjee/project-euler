use euler_rust::utils::parse;
use std::cmp;
use std::collections::HashSet;

fn main() {
    let largest = largest_pandigital_multiple(HashSet::from([1u32, 2, 3, 4, 5, 6, 7, 8, 9]));
    println!("{}", largest);
}

fn largest_pandigital_multiple(digit_set: HashSet<u32>) -> u64 {
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
	if let Some(m) = pandigital_product(n, &digit_set) {
	    largest = cmp::max(largest, m);
	    largest_first = parse::first_digit(largest) as u32;
	}
    }
    largest
}

fn pandigital_product(n: u32, digit_set: &HashSet<u32>) -> Option<u64> {
    let l = digit_set.len();
    let mut candidate_set: HashSet<u32> = HashSet::new();
    let mut candidate_vec: Vec<u32> = Vec::new();
    let mut failed = false;
    let mut m = 1;
    // Loop over multiplicands until we find a match or a failure.
    loop {
	// Loop over the digits of the product.
	for digit in parse::split_number_to_digits::<u32, u32>(n*m).into_iter() {
	    // If the digit is not in our pandigital set, or we've
	    // encountered it already, declare failure for this
	    // multiplicand.
	    if !digit_set.contains(&digit) || candidate_set.contains(&digit) {
		failed = true;
		break;
	    }
	    candidate_vec.push(digit);
	    candidate_set.insert(digit);
	}
	if failed {
	    return None
	}
	// We have a match!
	if candidate_set.len() == l {
	    return Some(parse::parse_slice_as_number::<u32, u64>(candidate_vec.as_slice()))
	}
	m += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pandigital_product() {
	let digits = HashSet::from([1u32, 2, 3, 4, 5, 6, 7, 8, 9]);
	assert_eq!(pandigital_product(192, &digits), Some(192384576));
	assert_eq!(pandigital_product(9, &digits), Some(918273645));
    }
}
