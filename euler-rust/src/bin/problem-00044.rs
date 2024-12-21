fn main() {
    let diff = smallest_pentagonal_difference();
    println!("{}", diff);
}

// Loop over the pentagonal numbers. For each new number, loop over
// all the previous pentagonal numbers and check the sum and
// difference for pentagonal-ity. The first pair found is our answer.
fn smallest_pentagonal_difference() -> u64 {
    let mut pentagonal_numbers = Vec::new();
    let mut p = 1u64;
    let mut diff = 4u64;
    loop {
	for q in pentagonal_numbers.iter() {
	    if is_pentagonal(p-q) && is_pentagonal(p+q) {
		return p-q
	    }
	}
	pentagonal_numbers.push(p);
	p += diff;
	diff += 3;
    }
}

// n is the m-th pentagonal number if m(3m-1)/2 = n. Using the
// quadratic formula, this means that m = (sqrt(1+24n) + 1)/6.
fn is_pentagonal(n: u64) -> bool {
    let s = 1 + 24 * n;
    let r = (s as f64).sqrt().floor() as u64;
    r * r == s && (r + 1) % 6 == 0
}
