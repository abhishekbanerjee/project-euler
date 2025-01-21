use euler_rust::utils::parse;

fn main() {
    let sum = sum_digit_factorials();
    println!("{}", sum);
}

fn sum_digit_factorials() -> u32 {
    let mut digit_factorials = vec![1u32; 10];
    // Compute factorials of the digits a-priori.
    for i in 1..10 {
	digit_factorials[i] = digit_factorials[i-1] * (i as u32);
    }
    // 9! * d < 10 ^ d for d > number of digits in 9!, so this is a
    // reasonable upper bound.
    let limit = digit_factorials[9] * (digit_factorials[9].to_string().len() as u32 + 1);
    (3..limit).filter(|&n| is_digit_factorial(n, digit_factorials.as_slice())).sum()
}

fn is_digit_factorial(n: u32, digit_factorials: &[u32]) -> bool {
    parse::split_number_to_digits::<u32, usize>(n).into_iter().map(|d| digit_factorials[d]).sum::<u32>() == n
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_digit_factorial() {
	let mut digit_factorials = vec![1u32; 10];
	for i in 1..10 {
	    digit_factorials[i] = digit_factorials[i-1] * (i as u32);
	}
	assert!(is_digit_factorial(145, digit_factorials.as_slice()));
    }
}

