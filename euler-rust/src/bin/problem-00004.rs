fn main() {
    let palindrome = largest_palindrome(3);
    println!("{}", palindrome);
}

fn largest_palindrome(num_digits: u32) -> u32 {
    let mut largest = 0u32;
    let (lower, upper) = (10u32.pow(num_digits-1), 10u32.pow(num_digits));
    // Take all three digit numbers and multiply them.
    for a in lower..upper {
	for b in a..upper {
	    let product = a*b as u32;
	    if product > largest && is_palindrome(product) {
		largest = product;
	    }
	}
    }
    largest
}

fn is_palindrome(n: u32) -> bool {
    let mut reverse = 0u32;
    let mut forward = n;
    while forward > 0 {
	reverse = reverse*10 + forward%10;
	forward = forward/10;
    }
    reverse == n
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_palindrome() {
	assert_eq!(largest_palindrome(2), 9009);
    }

    #[test]
    fn test_is_palindrome() {
	assert!(is_palindrome(12321));
	assert!(!is_palindrome(12345));
    }
}
