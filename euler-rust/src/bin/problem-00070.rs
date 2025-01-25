use euler_rust::utils::nums;
use euler_rust::utils::parse;
use euler_rust::utils::primes::Primes;

fn main() {
    let num = min_totient_permutation(10usize.pow(7));
    println!("{}", num);
}

// The most minimal n/phi(n) is achieved for prime numbers. But
// because phi(n) = n-1 for prime numbers, they cannot be permutations
// of each other. The next best scenario is product of two (large)
// primes, so that's what we're going to look for.
fn min_totient_permutation(limit: usize) -> usize {
    // Since we're looking for large prime products, we're looking
    // somewhere close to the square root of the limit. We multiply by
    // 10 for some wiggle room.
    let root = nums::int_square_root(limit * 10);
    let primes: Vec<usize> = Primes::up_to(root+1).all_primes_below(root);
    let mut min_ratio = 2.0f64;
    let mut min_n = 0usize;
    for &p1 in primes.iter() {
	for &p2 in primes.iter() {
	    if p2 == p1 { break; }
	    let n = p1 * p2;
	    if n > limit { break; }
	    let t = (p1 - 1) * (p2 - 1);
	    let r = (n as f64) / (t as f64);
	    // Test the ratio first, because canonicalization is more
	    // expensive.
	    if r > min_ratio { continue; }
	    if parse::canonicalize(n) != parse::canonicalize(t) { continue; }
	    min_ratio = r;
	    min_n = n;
	}
    }
    min_n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_totient_permutation() {
	assert_eq!(min_totient_permutation(10usize.pow(5)), 75841);
    }
}
