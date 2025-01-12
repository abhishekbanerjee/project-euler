fn main() {
    let digits = large_non_mersenne_prime(10);
    println!("{}", digits);
}

// Perform the arithmetic operation specified in the question, using
// modular arithmetic to keep number sizes constrained.
fn large_non_mersenne_prime(digits: u32) -> u64 {
    let modulus = 10u64.pow(digits);
    mult(28_433, exponentiation(2, 7_830_457, modulus), modulus) + 1
}

// Use the fast exponentiation by squaring algorithm specified in
// https://en.wikipedia.org/wiki/Exponentiation_by_squaring#With_constant_auxiliary_memory.
fn exponentiation(mut x: u64, mut n: u64, modulus: u64) -> u64 {
    let mut y = 1u64;
    while n > 1 {
	if n & 1 != 0 {
	    y = mult(x, y, modulus);
	    n = n - 1;
	}
	x = mult(x, x, modulus);
	n = n / 2;
    }
    mult(x, y, modulus)
}

fn mult(a: u64, b: u64, modulus: u64) -> u64 {
    // Use double the space for the multiplication teporarily, before
    // shrinking it down after applying the modulus.
    ((a as u128 * b as u128) % modulus as u128) as u64
}
