use num_bigint::BigUint;

fn main() {
    let sum = sum_power_digits(2, 1000);
    println!("{}", sum);
}

fn sum_power_digits(n: u32, exponent: u32) -> u32 {
    let mut sum = 0u32;
    // Use the big-int library for the large exponentiation. Break
    // into the digits, iterate and sum the digits up.
    for digit in BigUint::from(n).pow(exponent).to_radix_be(10u32).iter() {
	sum += *digit as u32;
    }
    sum
}
