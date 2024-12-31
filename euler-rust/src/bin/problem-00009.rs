use euler_rust::utils::nums;

fn main() {
    let special = special_pythagorean_triplet(1_000);
    println!("{}", special);
}

// We iterate over all candidate (a,b) with b <= a and check if
// sqrt(a^2 + b^2) is an integer for our candidate c. Pythagorean
// triplets found, we check for the condition and return the correct
// one.
fn special_pythagorean_triplet(n: u32) -> u64 {
    for a in 1..=n {
	for b in 1..=a {
	    match root(a*a + b*b) {
		None => {},
		Some(c) => {
		    if a + b + c == n {
			return (a as u64) * (b as u64) * (c as u64);
		    }
		},
	    }
	}
    }
    // The question specifies that we're guaranteed a satisfying
    // triplet, so we will never get here.
    return 0;
}

fn root(n: u32) -> Option<u32> {
    let r = nums::int_square_root(n);
    if r*r == n {
	Some(r)
    } else {
	None
    }
}
