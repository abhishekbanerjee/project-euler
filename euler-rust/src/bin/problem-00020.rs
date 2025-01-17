use num_bigint::BigUint;

fn main() {
    let sum = sum_factorial_digits(100);
    println!("{}", sum);
}

fn sum_factorial_digits(n: u32) -> u32 {
    let mut sum = 0u32;
    // Use the big-int library to store the factorial. Break into the
    // digits, iterate and sum the digits up.
    for digit in factorial(n).to_radix_be(10u32).iter() {
	sum += *digit as u32;
    }
    sum
}

fn factorial(n: u32) -> BigUint {
    let mut product = BigUint::from(1u32);
    for i in 1..=n {
	product *= i;
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_factorial_digits() {
	assert_eq!(sum_factorial_digits(10), 27);
    }
}
