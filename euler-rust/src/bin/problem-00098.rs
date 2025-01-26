use euler_rust::utils::combinations;
use euler_rust::utils::files;
use euler_rust::utils::parse;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let num = largest_square_anagram("resources/0098_words.txt");
    println!("{}", num);
}

fn largest_square_anagram(file_path: &str) -> u64 {
    let mut largest_anagram = 0u64;
    // Create a map of words by canonicalized version. This buckets
    // anagrams together. Also, keep track of the longest word found
    // so far.
    let mut canonicalized: HashMap<String, Vec<String>> = HashMap::new();
    let mut max_word_len = 0usize;
    for word in files::parse_words(file_path).into_iter() {
	max_word_len = cmp::max(max_word_len, (&word).chars().count());
	let canonical = canonicalize(word.as_str());
	if let Some(vec) = canonicalized.get_mut(&canonical) {
	    vec.push(word);
	} else {
	    canonicalized.insert(canonical, vec![word]);
	}
    }
    // Build a map of all square numbers by the number of digits in
    // the number, up to the maximum length of words in the file.
    let squares_by_len = gen_squares_by_len(max_word_len);
    // For all the words in the map,
    for words in canonicalized.values() {
	// ... make sure to only consider words with anagrams, and
	if words.len() > 1 {
	    // ... for each pair of anagrams, find a square match, if
	    // possible, and track the largest found so far.
	    for pair in combinations::combinations(words.as_slice(), 2) {
		let anagram = anagram_square_pair(&pair[0], &pair[1], &squares_by_len[pair[0].chars().count()]);
		largest_anagram = cmp::max(anagram, largest_anagram);
	    }
	}
    }
    // Return the largest.
    largest_anagram
}

fn anagram_square_pair(word1: &str, word2: &str, squares: &HashSet<u64>) -> u64 {
    let mut largest = 0u64;
    // Iterate over all square numbers with the same number of digits
    // as the words. These are our candidates for creating the char ->
    // digit maps.
    for &sq in squares.iter() {
	let mut digit_map = vec![' '; 10];
	let mut valid = true;
	// We use word1 to generate a digit -> char map.
	for (&d, c) in parse::split_number_to_digits::<u64, usize>(sq).iter().zip(word1.chars()) {
	    // For each mapping, ensure that no two letters have the
	    // same digit mapping.
	    if digit_map[d] != ' ' && digit_map[d] != c {
		valid = false;
		break;
	    }
	    digit_map[d] = c;
	}
	if !valid { continue; }
	// Convert the digit -> char to a char -> digit map.
	let char_map: HashMap<char, usize> = digit_map
	    .into_iter()
	    .enumerate()
	    .filter(|(_, c)| *c != ' ')
	    .map(|(i, c)| (c, i))
	    .collect();
	// Use the generated map to create the number corresponding to
	// word2.
	let n2: u64 = parse::parse_slice_as_number(
	    word2.chars()
		.map(|c| char_map[&c])
		.collect::<Vec<_>>()
		.as_slice());
	// If the number for word2 is also a square of appropriate
	// length, we have a match! Track the largest number found so
	// far.
	if squares.contains(&n2) {
	    largest = cmp::max(largest, cmp::max(sq, n2));
	}
    }
    largest
}

fn gen_squares_by_len(max_len: usize) -> Vec<HashSet<u64>> {
    let mut squares_by_len: Vec<HashSet<u64>> = vec![HashSet::new(); max_len+1];
    let mut base = 1u64;
    loop {
	let sq = base * base;
	let len = parse::count_digits(sq);
	if len > max_len { break; }
	squares_by_len[len].insert(sq);
	base += 1;
    }
    squares_by_len
}

// Sort the characters in the string to canonicalize it. This makes
// sure that anagrams result in the same canonicalization.
fn canonicalize(word: &str) -> String {
    let mut char_vec: Vec<char> = word.chars().collect();
    char_vec.sort();
    char_vec.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_square_pair() {
	// 4 digit squares.
	let squares: HashSet<u64> = (32u64..100).map(|n| n * n).collect(); 
	assert_eq!(anagram_square_pair("CARE", "RACE", &squares), 9216);
    }
}
