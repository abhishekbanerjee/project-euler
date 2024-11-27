fn main() {
    let palindrome = largest_palindrome();
    println!("{}", palindrome);
}

fn largest_palindrome() -> u32 {
    let mut largest = 0u32;
    for a in 100..1000 {
	for b in 100..1000 {
	    let product = a*b as u32;
	    if product > largest && is_palindrome(product) {
		largest = product;
	    }
	}
    }
    return largest
}

fn is_palindrome(n: u32) -> bool {
    let mut reverse = 0u32;
    let mut forward = n;
    while forward > 0 {
	reverse = reverse*10 + forward%10;
	forward = forward/10;
    }
    return reverse == n;
}
