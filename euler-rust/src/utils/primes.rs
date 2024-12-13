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
	return self.primes[index].clone();
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
