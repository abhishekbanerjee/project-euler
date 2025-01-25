use euler_rust::utils::parse;

fn main() {
    let smallest = permuted_multiples(6u8);
    println!("{}", smallest);
}

// For each number, compare the canonicalized versions of it and its
// multiples. Output the one where they are all the same.
fn permuted_multiples(multiplier_limit: u8) -> usize {
    let mut candidate = 1usize;
    loop {
	// If the first digit of the number is not 1, 6 times the
	// number will have more digits than the number itself.
	if parse::first_digit(candidate) == 1 {
	    let canonical = parse::canonicalize(candidate);
	    let mut is_successful = true;
	    for m in 2..=multiplier_limit {
		if parse::canonicalize(candidate * (m as usize)) != canonical {
		    is_successful = false;
		    break;
		}
	    }
	    if is_successful {
		return candidate
	    }
	}
	candidate += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permuted_multiples() {
	assert_eq!(permuted_multiples(2), 125874);
    }
}
