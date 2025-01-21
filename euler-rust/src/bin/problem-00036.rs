fn main() {
    let sum = sum_double_palindromes(1_000_000);
    println!("{}", sum);
}

// Convert the decimal and binary representations to string, compare
// the string characters to the reversed string characters.
fn sum_double_palindromes(limit: usize) -> usize {
    (1..limit).filter(|&n| is_double_palindrome(n)).sum()
}

fn is_double_palindrome(n: usize) -> bool {
    let n_dec = n.to_string();
    if !n_dec.chars().eq(n_dec.chars().rev()) { return false }
    let n_bin = format!("{n:b}");
    return n_bin.chars().eq(n_bin.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_double_palindrome() {
	assert!(is_double_palindrome(585));
    }
}
