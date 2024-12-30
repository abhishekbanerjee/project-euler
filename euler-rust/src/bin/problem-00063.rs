use euler_rust::utils::parse;
use num_bigint::BigUint;

fn main() {
    let count = powerful_digit_counts();
    println!("{}", count);
}

fn powerful_digit_counts() -> u64 {
    let mut power = 1u32;
    let mut threshold = 1u64;
    let mut count = 0u64;
    // The loop invariant is that threshold is the lowest single digit
    // number n which satisfies n^power is power-digit long.
    loop {
	// We know that 10^n has n+1 digits for every n. So, as soon
	// as we get to a power n where even 9^n does not have n
	// digits, we can stop.
	while threshold < 10u64 {
	    let p = BigUint::from(threshold).pow(power);
	    if parse::count_digits(p) as u32 == power { break; }
	    threshold += 1;
	}
	if threshold == 10u64 { break; }
	// If a single digit number n satisfies n^p being p-digit,
	// then all single digit numbers > n also satisfy this
	// condition. We need to count all of them.
	count += 10u64 - threshold;
	power += 1;
    }
    count
}
