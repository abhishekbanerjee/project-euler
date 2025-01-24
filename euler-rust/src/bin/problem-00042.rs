use euler_rust::utils::files;
use euler_rust::utils::nums;

fn main() {
    let count = triangular_words("resources/0042_words.txt");
    println!("{}", count);
}

// 1. Parse words. 2. Score words. 3. Check triangular and count.
fn triangular_words(file_path: &str) -> u32 {
    files::parse_words(file_path)
	.into_iter()
	.map(|w| is_triangular_word(w.as_str()) as u32)
	.sum()
}

fn is_triangular_word(word: &str) -> bool {
    // Score the word by subtracting 64 from the ascii value of each
    // letter (this translates 'A' to 1 and so on) of the word,
    // summing it all up.
    let score = word.chars().map(|c| c as u64 - 64).sum::<u64>();
    nums::is_triangular(score)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_triangular_word() {
	assert!(is_triangular_word("SKY"));
    }
}
