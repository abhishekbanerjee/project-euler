use euler_rust::utils::files;
use euler_rust::utils::nums;

fn main() {
    let count = triangular_words("resources/0042_words.txt");
    println!("{}", count);
}

// 1. Parse words. 2. Score words. 3. Check triangular and count.
fn triangular_words(file_path: &str) -> u32 {
    let mut count = 0u32;
    let words = files::parse_words(file_path);
    for word in words.iter() {
	let score = score_word(word);
	if nums::is_triangular(score as u64) { count += 1; }
    }
    count
}

// Score the word by subtracting 64 from the ascii value of each
// letter (this translates 'A' to 1 and so on) of the word, summing it
// all up.
fn score_word(word: &String) -> u32 {
    word.chars().map(|c| c as u32 - 64).sum::<u32>()
}
