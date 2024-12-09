fn main() {
    let sum = non_abundant_sums(28_123);
    println!("{}", sum);
}

// We use a sieve type method to find all non-abundant-sum numbers to
// sum up. We find all the abundant numbers in the range, and then run
// a nested loop to sum them and mark those off. Lastly, we sum up
// what's left and return the answer.
fn non_abundant_sums(limit: u32) -> u64 {
    let mut abundant_sums = vec![false; (limit+1) as usize];
    let abundant: Vec<u32> = (2..=limit).filter(|&n| is_abundant(n)).collect();
    for (i, n) in abundant.iter().enumerate() {
	for m in abundant[i..].iter() {
	    let s = m+n;
	    if s > limit { break; }
	    abundant_sums[s as usize] = true;
	}
    }
    abundant_sums
	.iter()
	.enumerate()
	.filter(|&(_, is_abundant_sum)| !*is_abundant_sum)
	.map(|(n, _)| n as u64)
	.sum()
}

fn is_abundant(n: u32) -> bool {
    euler_rust::divisors_sum(n) > n
}
