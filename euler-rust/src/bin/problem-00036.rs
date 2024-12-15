fn main() {
    let sum = sum_double_palindromes(1_000_000);
    println!("{}", sum);
}

// Convert the decimal and binary representations to string, compare
// the string characters to the reversed string characters.
fn sum_double_palindromes(limit: usize) -> usize {
    let mut sum = 0usize;
    for n in 1..limit {
	let n_dec = n.to_string();
	if !n_dec.chars().eq(n_dec.chars().rev()) { continue; }
	let n_bin = format!("{n:b}");
	if n_bin.chars().eq(n_bin.chars().rev()) {
	    sum += n;
	}
    }
    sum
}
