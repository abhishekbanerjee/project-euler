use num_traits::Zero;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;

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

// n is the m-th triangular number if m(m+1)/2 = n. Using the
// quadratic formula, this means that m = (sqrt(8n + 1) - 1)/2. So n
// is triangular if and only if 8n+1 is a perfect square.
pub fn is_triangular(n: u64) -> bool {
    let s = n * 8 + 1;
    let r = (s as f64).sqrt().floor() as u64;
    s == r * r
}

// n is the m-th pentagonal number if m(3m-1)/2 = n. Using the
// quadratic formula, this means that m = (sqrt(1+24n) + 1)/6.
pub fn is_pentagonal(n: u64) -> bool {
    let s = 1 + 24 * n;
    let r = (s as f64).sqrt().floor() as u64;
    r * r == s && (r + 1) % 6 == 0
}
