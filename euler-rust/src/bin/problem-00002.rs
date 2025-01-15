fn main() {
    let sum = sum_fibonaccis(4_000_000);
    println!("{}", sum);
}

fn sum_fibonaccis(n: u64) -> u64 {
    let mut sum = 0u64;
    let mut a = 1u64;
    let mut b = 2u64;
    while b <= n {
	// Check for the condition on the current number and update
	// the sum.
	if b % 2 == 0 {
	    sum += b;
	}
	// Keep the last 2 Fibonacci numbers around, and use them to
	// compute the next one.
	let c = a + b;
	a = b;
	b = c;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_fibonaccis() {
	assert_eq!(sum_fibonaccis(100), 44);
    }
}
