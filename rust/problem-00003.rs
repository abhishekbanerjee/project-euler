fn main() {
    let prime_factor = largest_prime_factor(600_851_475_143);
    println!("{}", prime_factor);
}

fn largest_prime_factor(n: u64) -> u64 {
    let root = (n as f64).sqrt().floor() as usize;
    let mut largest = 0u64;
    let mut sieve = vec![true; root+1];
    for factor in 2..=root {
	if !sieve[factor] {
	    continue;
	}
	let mut mark = factor*2;
	while mark <= root {
	    sieve[mark] = false;
	    mark += factor;
	}
	if n % (factor as u64) == 0 {
	    largest = factor as u64;
	}
    }
    if largest == 0 {
	return n
    }
    return largest
}
