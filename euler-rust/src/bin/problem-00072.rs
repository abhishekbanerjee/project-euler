use euler_rust::utils::primes::Primes;

fn main() {
    let count = fractions_count(1_000_000);
    println!("{}", count);
}

// The number of reduced fractions less than 1 for a given denominator
// d is equal to all numbers less than d which are co-prime to d. The
// count of these is the totient function. So, we want to find the sum
// of the totient functions of all numbers up to the limit.
fn fractions_count(limit: usize) -> usize {
    // A collection of all primes up to the limit.
    let primes = Primes::up_to(limit+2).all_primes_below(limit+1);
    // Starting off with totients[i] = i.
    let mut totients: Vec<usize> = (0..=limit).collect();
    // Manually setting phi(1).
    totients[1] = 0;
    // For each prime p, we will multiply the totients vector value
    // for all its multiples with (p-1) / p. Once we're done with all
    // the primes, totients[i] = phi(i) for all i > 0, by the totient
    // formula. totients[0] is already 0, so should not affect our
    // final sum.
    for &p in primes.iter() {
	let mut idx = p;
	while idx <= limit {
	    totients[idx] = totients[idx] / p * (p - 1);
	    idx += p;
	}
    }
    // Sum up the totients.
    totients.into_iter().sum()
}
