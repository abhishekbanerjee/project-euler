use euler_rust::utils::nums;

fn main() {
    let num = triangular_number_with_divisors(500);
    println!("{}", num);
}

fn triangular_number_with_divisors(d: u32) -> u64 {
    let mut seed = 2u64;
    // We run an infinite loop until we find a solution satisfying the
    // condition.
    loop {
	// We find the n-th triangular number by using the formmula
	// for the sum of the first n numbers.
	let num = if seed % 2 == 0 { seed/2*(seed+1) } else { (seed+1)/2*seed };
	// nums::divisors does not return 1 and the number itself, so
	// we need to account for that.
	if nums::divisors(num).len() as u32 > d - 2 {
	    return num
	}
	seed += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangular_number_with_divisors() {
	assert_eq!(triangular_number_with_divisors(5), 28);
    }
}
