use euler_rust::utils::parse;
use num_bigint::BigUint;

fn main() {
    let count = lychrel_numbers_below(10_000);
    println!("{}", count);
}

fn lychrel_numbers_below(limit: u32) -> u32 {
    (1..limit).map(|n| is_lychrel(BigUint::from(n)) as u32).sum()
}

// Keep iterating, reversing the number and adding to itself, until we
// find a palindrome or run out of iterations.
fn is_lychrel(n: BigUint) -> bool {
    let mut test = n.clone() + parse::reverse_number(n);
    for _ in 0..50 {
	let rev = parse::reverse_number(test.clone());
	if rev == test { return false }
	test = rev + test
    }
    true
}
