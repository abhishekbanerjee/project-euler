use num_traits::Zero;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::str::FromStr;

// Sums up all the divisors of a number except for itself.
pub fn divisors_sum(n: u32) -> u32 {
    if n == 1 { return 0; }
    // The square root is the largest possible non-identity divisor of
    // a number.
    let root = (n as f64).sqrt().ceil() as u32;
    // Start the count at 1, because 1 always divides the
    // number.
    let mut divisors_sum = 1u32;
    for d in 2..root {
	// Each number d evenly dividing n gets two distinct divisors:
	// d and n/d. Since d < sqrt(n) by the loop condition,
	// therefore n/d > sqrt(n) and we therefore would not
	// encounter n/d in this loop.
	if n % d == 0 {
	    divisors_sum += d + n/d;
	}
    }
    // If n is an even square, the square root counts only once.
    if n == root*root {
	divisors_sum += root;
    }
    divisors_sum
}

// We use the formula: LCM(a, b) = a * b / GCD(a, b), where GCD is the
// greatest common divisor.
pub fn lcm<T: Div<Output = T> + Mul<Output = T> + Rem<Output = T> + PartialOrd + Zero + Copy>(a: T, b: T) -> T {
    return a*b/gcd(a,b);
}

// We use the resursive remainder method to find GCD.
// TODO: make this iterative.
pub fn gcd<T: Rem<Output = T> + PartialOrd + Zero + Copy>(a: T, b: T) -> T {
    if a > b {
	return gcd(b, a)
    }
    if a == T::zero() {
	return b
    }
    gcd(b % a, a)
}

// Splits the digits of the number written out in the given base.
pub fn split_digits<T: Clone + Copy + Div<Output = T> + PartialEq + Rem<Output = T> + Zero>(n: T, base: T) -> Vec<T> {
    let mut m = n.clone();
    let mut digits = Vec::new();
    while m != T::zero() {
	digits.insert(0, m % base);
	m = m / base;
    }
    digits
}

// Converts the digits (in the given base) to the constituent number.
pub fn collect_digits<T: Add + Copy + Mul<Output = T> + Zero>(digits: &[T], base: T) -> T {
    let mut n = T::zero();
    for digit in digits.iter() {
	n = n * base + *digit;
    }
    n
}

// Extracts the first digit of the given number.
pub fn first_digit<T: Display + FromStr<Err: Debug> + ToString>(n: T) -> T {
    nth_digit(n, 0usize)
}

// Extracts the n-th (0-based) digit of the given number (written from
// left to right).
pub fn nth_digit<T: Display + FromStr<Err: Debug> + ToString>(number: T, idx: usize) -> T {
    number
	.to_string()[(idx)..(idx+1)]
	.parse::<T>()
	.expect(format!("{} does not have a {}-th digit!", number, idx).as_str())
}
