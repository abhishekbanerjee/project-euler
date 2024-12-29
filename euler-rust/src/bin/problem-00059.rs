use euler_rust::utils::files;

fn main() {
    let sum = xor_decyption_sum("resources/0059_cipher.txt");
    println!("{}", sum);
}

// Iterate over the key combinations, decrypt the text and score
// it. Accept the decryption with the maximum score.
fn xor_decyption_sum(file_name: &str) -> u32 {
    let encrypted: Vec<u8> = files::read_file(file_name)
	.split(",")
	.map(|s| s.parse::<u8>().unwrap())
	.collect();
    let mut max_score = 0u32;
    let mut best_decryption: Vec<u8> = Vec::new();
    // Iterate over all lowercase character combinations for potential
    // keys.
    for k1 in 97u8..=122u8 {
	for k2 in 97u8..=122u8 {
	    for k3 in 97u8..=122u8 {
		let key = vec![k1,k2,k3];
		let decrypted = decrypt(encrypted.as_slice(), key.as_slice());
		match score_text(decrypted.as_slice()) {
		    None => (),
		    Some(score) => {
			if score > max_score {
			    max_score = score;
			    best_decryption = decrypted;
			}
		    },
		}
	    }
	}
    }
    best_decryption.into_iter().map(|c| c as u32).sum()
}

// Simple decryption algorithm. Go over the encrypted string, XOR-ing
// the current character with the corresponding key character, cycling
// the key over when it's exhausted.
fn decrypt(encrypted: &[u8], key: &[u8]) -> Vec<u8> {
    let period = key.len();
    let mut decrypted = Vec::new();
    for (i, c) in encrypted.iter().enumerate() {
	decrypted.push(*c ^ key[i % period]);
    }
    decrypted
}

// Simple scoring algorithm: count all the alphabets, and disqualify
// the text if it contains a non-printable character.
fn score_text(text: &[u8]) -> Option<u32> {
    let mut score = 0u32;
    for c in text.iter() {
	// If the character is out of the printable range, disqualify
	// the text.
	if *c < 32u8 || *c > 126u8 {
	    return None
	}
	// If the character is alphabetical, add it to the score.
	if *c >= 65u8 && *c <= 90u8 || *c >= 97 && *c <= 122u8 {
	    score += 1;
	}
    }
    Some(score)
}
