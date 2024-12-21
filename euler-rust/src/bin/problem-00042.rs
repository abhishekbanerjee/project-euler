use euler_rust::utils::files;

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
	if is_triangular(score) { count += 1; }
    }
    count
}

// Score the word by subtracting 64 from the ascii value of each
// letter (this translates 'A' to 1 and so on) of the word, summing it
// all up.
fn score_word(word: &String) -> u32 {
    word.chars().map(|c| c as u32 - 64).sum::<u32>()
}

// n is the m-th triangular number if m(m+1)/2 = n. Using the
// quadratic formula, this means that m = (sqrt(8n + 1) - 1)/2. So n
// is triangular if and only if 8n+1 is a perfect square.
fn is_triangular(n: u32) -> bool {
    let s = n * 8 + 1;
    let r = (s as f32).sqrt().floor() as u32;
    s == r * r
}
