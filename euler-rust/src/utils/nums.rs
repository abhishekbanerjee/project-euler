use num_traits::FromPrimitive;
use num_traits::One;
use num_traits::ToPrimitive;
use num_traits::Zero;
use std::iter::Sum;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;

// Returns all divisors of a number except for 1 and itself.
pub fn divisors<T: Copy + Div<Output = T> + FromPrimitive + Mul<Output = T> + One + PartialEq + PartialOrd + Rem<Output = T> + ToPrimitive + Zero>(n: T) -> Vec<T> {
    let mut divs: Vec<T> = Vec::new();
    let root = int_square_root(n);
    let mut d = T::one();
    while d <= root {
	if (n % d).is_zero() && !d.is_one() {
	    divs.push(d);
	    if !(d == root && n == root * root) { divs.push(n/d); }
	}
	d = d + T::one();
    }
    divs
}

// Sums up all the divisors of a number except for itself (and
// including 1, unless the number is 1).
pub fn divisors_sum<T: Copy + Div<Output = T> + FromPrimitive + Mul<Output = T> + One + PartialEq + PartialOrd + Rem<Output = T> + Sum + ToPrimitive + Zero>(n: T) -> T {
    if n.is_one() { T::zero() } else { T::one() + divisors(n).into_iter().sum() }
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

pub fn int_square_root<T: Copy + FromPrimitive + Mul<Output = T> + PartialEq + ToPrimitive>(n: T) -> T {
    T::from_f64(n.to_f64().unwrap().sqrt().floor()).unwrap()
}

pub fn is_perfect_square<T: Copy + FromPrimitive + Mul<Output = T> + PartialEq + ToPrimitive>(n: T) -> bool {
    let r = int_square_root(n);
    n == r*r
}

// n is the m-th triangular number if m(m+1)/2 = n. Using the
// quadratic formula, this means that m = (sqrt(8n + 1) - 1)/2. So n
// is triangular if and only if 8n+1 is a perfect square.
pub fn is_triangular(n: u64) -> bool {
    let s = n * 8 + 1;
    is_perfect_square(s)
}

// n is the m-th pentagonal number if m(3m-1)/2 = n. Using the
// quadratic formula, this means that m = (sqrt(1+24n) + 1)/6.
pub fn is_pentagonal(n: u64) -> bool {
    let s = 1 + 24 * n;
    let r = (s as f64).sqrt().floor() as u64;
    r * r == s && (r + 1) % 6 == 0
}
