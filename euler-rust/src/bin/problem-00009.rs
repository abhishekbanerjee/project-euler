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
	    if let Some(c) = root(a*a + b*b) {
		if a + b + c == n {
		    return (a as u64) * (b as u64) * (c as u64)
		}
	    }
	}
    }
    // The question specifies that we're guaranteed a satisfying
    // triplet, so we will never get here.
    panic!("Impossible!");
}

fn root(n: u32) -> Option<u32> {
    let r = nums::int_square_root(n);
    if r*r == n {
	Some(r)
    } else {
	None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_pythagorean_triplet() {
	assert_eq!(special_pythagorean_triplet(12), 60);
    }

    #[test]
    #[should_panic]
    fn test_special_pythagorean_triplet_fail() {
	special_pythagorean_triplet(10);
    }
}
