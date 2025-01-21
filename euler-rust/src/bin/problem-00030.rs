fn main() {
    let sum = sum_powers(5);
    println!("{}", sum);
}

fn sum_powers(power: u32) -> u32 {
    // 9^power * n < 10^n for n = power+1, so this is a reasonable
    // upper limit.
    let limit = 10u32.pow(power + 1);
    let mut sum = 0u32;
    let mut digit_powers = Vec::new();
    // First compute the relevant power of all single digits. We do
    // this once and just sum it up for all numbers we consider.
    for digit in 0..10 {
	digit_powers.push((digit as u32).pow(power));
    }
    // Loop through all candidate numbers. Sum power of digits and
    // check if it satisfies the condition.
    for n in 10..=limit {
	if n == sum_power_digits(n.clone(), digit_powers.as_slice()) {
	    sum += n;
	}
    }
    return sum;
}

fn sum_power_digits(mut n: u32, digit_powers: &[u32]) -> u32 {
    let mut sum = 0u32;
    while n != 0 {
	sum += digit_powers[(n % 10) as usize];
	n /= 10;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_powers() {
	assert_eq!(sum_powers(4), 19316);
    }
}
