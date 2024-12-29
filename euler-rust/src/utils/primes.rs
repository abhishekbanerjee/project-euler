use std::collections::HashSet;

const DEFAULT_SIZE : usize = 100_000;

pub struct Primes {
    sieve: Vec<bool>,
    primes: Vec<usize>,
}

impl Primes {
    pub fn new() -> Primes {
	Primes::up_to(DEFAULT_SIZE) 
    }

    pub fn up_to(initial_size: usize) -> Primes {
	let mut sieve = vec![true; initial_size];
	sieve[0] = false;
	sieve[1] = false;
	let mut primes: Vec<usize> = Vec::new();
	Self::set_up(sieve.as_mut_slice(), &mut primes, 2);
	Primes { sieve, primes }
    }

    pub fn prime_number(&mut self, index: usize) -> usize {
	while self.primes.len() <= index {
	    self.grow();
	}
	self.primes[index].clone()
    }

    // Grows the sieve to the given candidate first. If the sieve is
    // not already that big, consider using the is_prime_fast method
    // instead, which might not grow the sieve that much.
    pub fn is_prime(&mut self, prime_candidate: usize) -> bool {
	while self.sieve.len() <= prime_candidate {
	    self.grow();
	}
	self.sieve[prime_candidate].clone()
    }

    // Checks all primes till the square root of the number to find a
    // divisor. Use this when the numbers can grow to much bigger than
    // the sieve.
    pub fn is_prime_fast(&mut self, prime_candidate: usize) -> bool {
	if self.sieve.len() > prime_candidate {
	    return self.is_prime(prime_candidate)
	}
	if prime_candidate % 2 == 0 { return false }
	let mut idx = 1usize;
	loop {
	    let prime = self.prime_number(idx);
	    if prime_candidate % prime == 0 { return false }
	    if prime * prime > prime_candidate { return true }
	    idx += 1;
	}
    }

    pub fn all_primes_below(&mut self, limit: usize) -> Vec<usize> {
	while self.sieve.len() <= limit {
	    self.grow();
	}
	let idx = match self.primes.binary_search(&limit) {
	    Ok(i) => i,
	    Err(i) => i,
	};
	self.primes[..idx].to_vec()
    }

    pub fn prime_factors(&mut self, mut n: usize) -> HashSet<usize> {
	let mut factors: HashSet<usize> = HashSet::new();
	if self.is_prime(n) {
	    factors.insert(n);
	} else {
	    let mut prime_idx = 0usize;
	    while n != 1 {
		let prime = self.prime_number(prime_idx);
		while n % prime == 0 {
		    factors.insert(prime);
		    n /= prime;
		}
		prime_idx += 1;
	    }
	}
	factors
    }


    fn set_up(sieve: &mut [bool], primes: &mut Vec<usize>, start: usize) {
	for current_factor in start..sieve.len() {
	    if sieve[current_factor] {
		primes.push(current_factor);
		Self::mark(sieve, current_factor, current_factor+1);
	    }
	}
    }

    fn grow(&mut self) {
	let length = self.sieve.len();
	self.sieve.extend(vec![true; length]);
	for prime in self.primes.iter() {
	    Self::mark(self.sieve.as_mut_slice(), *prime, length);
	}
	Self::set_up(self.sieve.as_mut_slice(), &mut self.primes, length);
    }

    fn mark(sieve: &mut [bool], prime: usize, start_at: usize) {
	let mut idx = (((start_at as f64)/(prime as f64)).ceil() as usize) * prime;
	while idx < sieve.len() {
	    sieve[idx] = false;
	    idx += prime;
	}
    }
}
