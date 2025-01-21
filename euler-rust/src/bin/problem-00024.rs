use euler_rust::utils::parse;
use euler_rust::utils::permutations::Permutations;

fn main() {
    let permutation = lexicographic_permutation_number(1_000_000);
    println!("{}", permutation);
}

fn lexicographic_permutation_number(n: u32) -> String {
    let mut permutations = Permutations::with((0..10).collect::<Vec<u8>>().as_slice());
    // Move forward n-1 times and don't collect the output.
    for _ in 0..n-1 { permutations.next_permutation(); }
    // Next permutation is the one we want. Collect its digits into a
    // string.
    parse::parse_slice_as_string(permutations.next_permutation().unwrap())
}

#[cfg(test)]
mod tests {
    use euler_rust::utils::parse;
    use euler_rust::utils::permutations::Permutations;

    #[test]
    fn test_is_abundant() {
	let mut permutations_generator = Permutations::with(&[0u8, 1, 2]);
	let mut permutations = Vec::new();
	loop {
	    match permutations_generator.next_permutation() {
		Some(p) => permutations.push(parse::parse_slice_as_string(p)),
		None => break,
	    }
	}
	assert_eq!(permutations, vec![String::from("012"),
				      String::from("021"),
				      String::from("102"),
				      String::from("120"),
				      String::from("201"),
				      String::from("210")]);
    }
}
