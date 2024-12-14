use std::collections::HashSet;
use euler_rust::utils::parse;
use euler_rust::utils::permutations::Permutations;

fn main() {
    let products = pandigital_products(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    println!("{}", products);
}

fn pandigital_products(digits: &[u8]) -> u32 {
    let mut permutations = Permutations::with(digits);
    let mut pandigital = HashSet::new();
    loop {
	match permutations.next_permutation() {
	    None => break,
	    Some(state) => {
		// There are two possible breakdowns of the digits
		// that could work. 4-digit = 1-digit x 4=digit, and
		// 4-digit = 2-digit x 3-digit.
		let prod: u32 = parse::parse_slice_as_number(&state[..4]);
		let (m11, m12) : (u32, u32) = (
		    parse::parse_slice_as_number(&state[4..5]),
		    parse::parse_slice_as_number(&state[5..]));
		let (m21, m22) : (u32, u32) = (
		    parse::parse_slice_as_number(&state[4..6]),
		    parse::parse_slice_as_number(&state[6..]));
		if prod == m11 * m12 || prod == m21 * m22 {
		    pandigital.insert(prod);
		}
	    },
	}
    }
    pandigital.into_iter().sum()
}
