use euler_rust::utils::nums;

fn main() {
    let sum = amicable_numbers_sum(10_000);
    println!("{}", sum);
}

fn amicable_numbers_sum(limit: u32) -> u64 {
    let mut amicable_sum = 0u64;
    for n in 2..limit {
	let d = nums::divisors_sum(n);
	// We eliminate all numbers for which d(n) > n, to prevent
	// double counting. We also eliminate d(n) = n because
	// amicable pairs are distinct by definition.
	if d >= n {
	    continue;
	}
	if nums::divisors_sum(d) == n {
	    amicable_sum += (d + n) as u64;
	}
    }
    amicable_sum
}

#[cfg(test)]
mod tests {
    use euler_rust::utils::nums;

    #[test]
    fn test_divisors_sum() {
	assert_eq!(nums::divisors_sum(220), 284);
	assert_eq!(nums::divisors_sum(284), 220);
    }
}
