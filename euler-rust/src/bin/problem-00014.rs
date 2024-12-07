fn main() {
    let longest = longest_collatz_under(1_000_000);
    println!("{}", longest);
}

fn longest_collatz_under(n: u32) -> u32 {
    let mut longest = 1u32;
    let mut longest_length = 0u64;
    for current in 2..n {
	let length = collatz_length(current);
	if length > longest_length {
	    longest = current;
	    longest_length = length;
	}
    }
    longest
}

// We find the length of the Collatz sequence by traversing the
// sequence. This obviously can be memoized, but we omit that exercise
// here.
fn collatz_length(start: u32) -> u64 {
    let mut n = start as u64;
    let mut length = 0;
    while n != 1 {
	n = if n % 2 == 0 { n/2 } else { 3*n+1 };
	length += 1;
    }
    length
}
