fn main() {
    let prime = find_prime_number(10_001);
    println!("{}", prime);
}

fn find_prime_number(n: u32) -> u64 {
    let mut sieve = vec![true; 100_000];
    let mut primes: Vec<u64> = Vec::new();
    let mut current_factor: u64 = 2;
    while (primes.len() as u32) < n {
	if current_factor >= sieve.len() as u64 {
	    let current_length = sieve.len();
	    double(&mut sieve);
	    for prime in primes.iter() {
		mark(sieve.as_mut_slice(), *prime, current_length as u64);
	    }
	}
	if sieve[current_factor as usize] {
	    primes.push(current_factor);
	    mark(sieve.as_mut_slice(), current_factor, current_factor + 1);
	}
	current_factor += 1;
    }
    return *primes.last().expect("No primes found!");
}

fn double(sieve: &mut Vec<bool>) {
    let length = sieve.len();
    sieve.extend(vec![true; length]);
}

fn mark(sieve: &mut [bool], prime: u64, start_at: u64) {
    let mut idx = (((start_at as f64)/(prime as f64)).ceil() as u64) * prime;
    while idx < (sieve.len() as u64) {
	sieve[idx as usize] = false;
	idx += prime;
    }
}
