use euler_rust::utils::files;

fn main() {
    let scores = names_scores("resources/0022_names.txt");
    println!("{}", scores);
}

// 1. Parse names. 2. Sort names. 3. Score names. 4. Sum scores.
fn names_scores(file_path: &str) -> u64 {
    let mut score = 0u64;
    let mut names = files::parse_words(file_path);
    names.sort();
    for (idx, name) in names.into_iter().enumerate() {
	score += score_name(idx as u32 + 1, &name) as u64;
    }
    score
}

// Score the name by subtracting 'A' from each letter and adding 1
// (this translates 'A' to 1 and so on) of the name, summing it all up
// and multiplying by the position of the name in the sorted array.
fn score_name(position: u32, name: &str) -> u32 {
    name.chars().map(|c| c as u32 - 'A' as u32 + 1).sum::<u32>() * position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_name() {
	assert_eq!(score_name(938, "COLIN"), 49714);
    }
}
