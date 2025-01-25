use euler_rust::utils::files;
use std::collections::HashSet;

fn main() {
    let count = poker_wins("resources/0054_poker.txt");
    println!("{}", count);
}

fn poker_wins(file_name: &str) -> u32 {   
    files::read_file(file_name).as_str().lines().map(|h| score_poker_hand(h)).sum()
}

// Scores the hand according to the poker rules in the
// question. Returns 1 if player 1 wins and 0 otherwise.
fn score_poker_hand(hand: &str) -> u32 {
    // Split up the hands string into the two players' hands.
    let hands: Vec<&str> = hand.split_whitespace().collect();
    let (player1, player2) = (&hands[..5], &hands[5..]);

    // Gather the info needed to score the hands.
    let ((counts1, values1), (counts2, values2)) = (card_values_score(&player1), card_values_score(&player2));
    let (suits1, suits2) = (suits_count(&player1), suits_count(&player2));

    // Find out the kind of hand and its score.
    let (score1, score2) = (primary_score(counts1.as_slice(), values1.as_slice(), suits1), primary_score(counts2.as_slice(), values2.as_slice(), suits2));

    // If the kind of hand is different, we have a winner.
    if score1 != score2 { return (score1 > score2) as u32 }

    // The kind of hand is the same, so we compare hand values.
    score_values(values1.as_slice(), values2.as_slice())
}

// Score the kind of hand this is. Higher ranking hands get a higher score.
fn primary_score(counts: &[u8], values: &[u8], num_suits: u8) -> u8 {
    if is_straight(values) && num_suits == 1 { return 8 } // Flushes, straight and royal
    if counts[0] == 4 { return 7 } // Four of a kind
    if counts[0] == 3 && counts[1] == 2 { return 6 } // Full house
    if num_suits == 1 { return 5 } // Flush
    if is_straight(values) { return 4 } // Straight
    if counts[0] == 3 { return 3 } // Three of a kind
    if counts[0] == 2 && counts[1] == 2 { return 2 } // Two pairs
    if counts[0] == 2 { return 1 } // One pair
    0 // Default (goes to high card)
}

fn score_values(values1: &[u8], values2: &[u8]) -> u32 {
    for (v1, v2) in values1.iter().zip(values2) {
	if v1 == v2 { continue; }
	return (v1 > v2) as u32
    }
    panic!("Completely equal hands!")
}

fn is_straight(values: &[u8]) -> bool {
    if values.len() < 5 { return false }
    let min = values.iter().min().unwrap();
    let canon: Vec<u8> = values.iter().map(|v| v-min).collect();
    // The first case is a regular straight. The second one is the A,
    // 2, 3, 4, 5 straight.
    vec![4, 3, 2, 1, 0].eq(&canon) || vec![12, 3, 2, 1, 0].eq(&canon)
}

// Count of unique suits in the hands.
fn suits_count(hand: &[&str]) -> u8 {
    let suits: HashSet<_> = hand.iter().map(|s| &s[1..]).collect();
    suits.len() as u8
}

// Returns two vectors: the counts of various values in the hands and
// the values themselves. Sorted in decreasing order by counts, and
// then values (since counts are more important for ranking purposes).
fn card_values_score(hand: &[&str]) -> (Vec<u8>, Vec<u8>) {
    let values = card_values(hand);
    let mut histogram = vec![0u8; 15];
    for value in values.iter() {
	histogram[*value as usize] += 1;
    }
    let mut score = Vec::new();
    for (i, count) in histogram.iter().enumerate() {
	if *count != 0 {
	    score.push((*count, i as u8));
	}
    }
    score.sort();
    score.reverse();
    score.into_iter().unzip()
}

fn card_values(hand: &[&str]) -> Vec<u8> {
    hand.iter().map(|s| card_value(&s[..1])).collect()
}

// Number cards get the number value, with other cards getting
// subsequently increasing values according to their rank. Ace gets
// 14, but we account for it in straights separately.
fn card_value(value: &str) -> u8 {
    match value.parse::<u8>() {
	Ok(val) => return val,
	_ => (),
    };
    match value {
	"T" => 10,
	"J" => 11,
	"Q" => 12,
	"K" => 13,
	"A" => 14,
	_ => panic!("Impossible!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_poker_hand() {
	assert_eq!(score_poker_hand("5H 5C 6S 7S KD 2C 3S 8S 8D TD"), 0);
	assert_eq!(score_poker_hand("5D 8C 9S JS AC 2C 5C 7D 8S QH"), 1);
	assert_eq!(score_poker_hand("2D 9C AS AH AC 3D 6D 7D TD QD"), 0);
	assert_eq!(score_poker_hand("4D 6S 9H QH QC 3D 6D 7H QD QS"), 1);
	assert_eq!(score_poker_hand("2H 2D 4C 4D 4S 3C 3D 3S 9S 9D"), 1);
    }
}
