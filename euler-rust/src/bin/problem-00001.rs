fn main() {
    let sum = sum_multiples(1000);
    println!("{}", sum);
}

fn sum_multiples(n: u32) -> u32 {
    let mut sum = 0u32;
    // Straight shot, loop through the numbers and check for the
    // condition, adding to the sum if satisfied.
    for i in 1..n {
	if i % 3 == 0 || i % 5 == 0 {
	    sum += i;
	}
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_multiples() {
	assert_eq!(sum_multiples(10), 23);
    }
}
