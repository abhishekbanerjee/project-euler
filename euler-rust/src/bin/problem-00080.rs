use euler_rust::utils::nums;
use euler_rust::utils::parse;
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;

fn main() {
    let sum = square_root_digits_sum(100, 100);
    println!("{}", sum);
}

fn square_root_digits_sum(limit: u8, digits: usize) -> u32 {
    let mut sum = 0u32;
    for n in 1..=limit {
	// Only irrational square roots needed.
	if nums::is_perfect_square(n) { continue; }
	sum += square_root_digits(n, digits).into_iter().sum::<u32>();
    }
    sum
}

// We run the long division algorithm to find the square root commonly
// taught in schools.
fn square_root_digits(n: u8, digits: usize) -> Vec<u32> {
    let (zero, one, two, ten, hundred) = big_numbers();
    let mut num_digits = parse::split_number_to_digits::<u8, u8>(n);
    if num_digits.len() % 2 == 1 { num_digits.insert(0, 0); }
    let mut decimals: Vec<u32> = Vec::new();
    // c represents the remainders.
    let mut c = zero.clone();
    // m represents the values on the quotient (left) side.
    let mut m = zero.clone();
    let mut i = 0usize;
    while decimals.len() < digits {
	// Bring down the next two digits (or zeros if we're past the
	// decimal point.
	c *= &hundred;
	if i < num_digits.len() {
	    c += parse::parse_slice_as_number::<u8, BigUint>(&num_digits[i..(i+2)]);
	    i += 2;
	}
	// x is the next digit in the decimal expansion. We grow x in
	// a simple loop while it stays less than the remainder we
	// need to subtract from.
	let mut x = zero.clone();
	loop {
	    if (&m * &ten + &x + &one) * (&x + &one) > c { break; }
	    x += &one;
	}
	// Update the remainder and quotients for the next step.
	c -= (&m * &ten + &x) * &x;
	m = &m * &ten + &x * &two;
	// Store the next digit.
	decimals.push(x.to_u32().unwrap());	
    }
    decimals
}

fn big_numbers() -> (BigUint, BigUint, BigUint, BigUint, BigUint) {
    (BigUint::ZERO, BigUint::from(1u8), BigUint::from(2u8), BigUint::from(10u8), BigUint::from(100u8))
}
