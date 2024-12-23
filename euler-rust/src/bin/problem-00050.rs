use euler_rust::utils::primes::Primes;

fn main() {
    let prime = consecutive_prime_sum(1_000_000);
    println!("{}", prime);
}

fn consecutive_prime_sum(limit: usize) -> usize {
    let mut largest = 0usize;
    let mut largest_seq = 0usize;
    let mut primes = Primes::up_to(limit+1);
    let mut prefix_sums = vec![0usize];
    let mut idx = 0usize;
    // Compute all the prefix sums until we get to the point where the
    // cumulative sum gets to the limit.
    loop {
	let sum = prefix_sums[idx] + primes.prime_number(idx);
	if sum >= limit { break; }
	prefix_sums.push(sum);
	idx += 1;
    }
    // Quadratic loop over the prefix sums.
    for i in 1..prefix_sums.len() {
	for j in 0..i {
	    // If the largest gap so far crosses the gap, break.
	    if i-j <= largest_seq { break; }
	    let candidate = prefix_sums[i] - prefix_sums[j];
	    // If the subset sum of primes in the gap is prime, we
	    // update the longest sequence found so far.
	    if candidate % 2 == 0 { continue; }
	    if primes.is_prime(candidate) {
		largest = candidate;
		largest_seq = i-j;
	    }
	}
    }
    largest
}
