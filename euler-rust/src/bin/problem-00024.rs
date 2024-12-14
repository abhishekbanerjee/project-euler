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
