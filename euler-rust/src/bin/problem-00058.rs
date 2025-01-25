use euler_rust::utils::primes::Primes;

fn main() {
    let length = spiral_primes_ratio(0.10);
    println!("{}", length);
}

// We move out from the center of the spiral, in a method very similar
// to Problem 28.
fn spiral_primes_ratio(threshold: f32) -> usize {
    let mut prime_count = 0usize;
    let mut primes = Primes::new();
    // The gap between adjacent diagonal elements. Grows by 2 every
    // time we move out in the spiral. Current side length is one more
    // than this number (accounting for a diagonal element and the gap
    // to the next one).
    let mut step = 2usize;
    let mut current = 1usize;
    loop {
	// Four diagonal elements in the current spiral.
	for _ in 0..4 {
	    current += step;
	    if primes.is_prime_fast(current) {
		prime_count += 1;
	    }
	}
	let denominator = (step * 2 + 1) as f32;
	if (prime_count as f32) / denominator < threshold {
	    return step + 1
	}
	step += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_primes_ratio() {
	assert_eq!(spiral_primes_ratio(0.5), 11);
    }
}
