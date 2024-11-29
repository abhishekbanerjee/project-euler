fn main() {
    let sum = sum_primes(2_000_000);
    println!("{}", sum);
}

fn sum_primes(n: u32) -> u64 {
    let mut sum = 0u64;
    let mut sieve = vec![true; (n+1) as usize];
    for factor in 2..=n {
	if !sieve[factor as usize] {
	    continue;
	}
	sum += factor as u64;
	let mut mark = factor*2;
	while mark <= n {
	    sieve[mark as usize] = false;
	    mark += factor;
	}
    }
    sum
}
