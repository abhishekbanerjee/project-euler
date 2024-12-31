use euler_rust::utils::nums;
use num_bigint::BigUint;

fn main() {
    let largest = largest_diophantine_x(1000);
    println!("{}", largest);
}

// The Diophantine equation in this question is Pell's equation. We
// use the method detailed in
// https://en.wikipedia.org/wiki/Pell%27s_equation#Fundamental_solution_via_continued_fractions
// to find the smallest x for each d in order, keeping track of the d
// with the largest x found so far.
fn largest_diophantine_x(limit: u32) -> u32 {
    let mut largest_x = BigUint::ZERO;
    let mut d_with_largest_x = 0u32;
    for d in 2..=limit {
	if nums::is_perfect_square(d) { continue; }
	let sequence = square_root_sequence(d);
	// Construct the right convergent fraction
	let x = pell_fundamental_x(sequence.as_slice());
	if x > largest_x {
	    largest_x = x.clone();
	    d_with_largest_x = d;
	}
    }
    d_with_largest_x
}

// Find the periodic sequence of coefficients in the continued
// fraction of the square root of the input number n. The first member
// of the output is the non-repeated integer square root, and the rest
// of the output is the repeated part.
//
// This is majorly borrowed from the solution to problem 64.
fn square_root_sequence(n: u32) -> Vec<u32> {
    let r = nums::int_square_root(n);
    let mut pair: (u32, u32) = (1, r);
    let mut sequence: Vec<u32> = vec![r];
    loop {
	let (old_x, old_y) = pair;
	let new_x = (n - old_y*old_y) / old_x;
	let a = (r + old_y) / new_x;
	sequence.push(a);
	let new_y = a * new_x - old_y;
	if new_x == 1 && new_y == r { break; }
	pair = (new_x, new_y);
    }
    sequence
}

// Given a sequence with the integer square root and the repeating
// fractional part of the square root of a number, constructs the
// convergent fraction up to the right length for the fundamental
// solution of Pell's equation, and returns the numerator of the
// fraction.
//
// This is majorly borrowed from the solution to problem 65. The only
// part which differs is choosing the right number of terms in the
// convergent fraction, and figuring out the actual current term of
// the sequence to use.
fn pell_fundamental_x(coefficients: &[u32]) -> BigUint {
    // Since the first term in the sequence is from the non-repeating
    // part, the period of the repeating part is length - 1.
    let period = coefficients.len() - 1;
    // The correct number of terms to use in the sequence, per the
    // article.
    let stop = if period % 2 == 0 { period - 1 } else { 2 * period - 1 };
    let (mut n, mut d) = (BigUint::from(1u8), BigUint::ZERO);
    for i in (0..=stop).rev() {
	let t = BigUint::from(coefficients[if i > period { i % period } else { i }]);
	let new_n = t * &n + d;
	d = n;
	n = new_n;
    }
    n.clone()
}
