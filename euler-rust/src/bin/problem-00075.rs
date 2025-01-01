use euler_rust::utils::nums;
use std::collections::HashSet;

fn main() {
    let count = singular_pythagorean_triples(1_500_000);
    println!("{}", count);
}

// We use a variant of Euclid's formula
// (https://en.wikipedia.org/wiki/Pythagorean_triple#A_variant) to
// generate all primitive Pythagorean triples. We use two hash sets to
// store singular and duplicate perimeters encountered.
fn singular_pythagorean_triples(limit: u32) -> u32 {
    let mut singulars: HashSet<u32> = HashSet::new();
    let mut duplicates: HashSet<u32> = HashSet::new();
    // The square root of the maximum perimeter is a reasonable limit
    // for the parameters of Euler's formula.
    for m in (1..nums::int_square_root(limit)).step_by(2) {
	for n in (1..m).step_by(2) {
	    // The parameters should be coprime for primitive triples.
	    if nums::gcd(n, m) != 1 { continue; }
	    let p = m*n + (m*m - n*n)/2 + (m*m + n*n)/2;
	    if p > limit { break; }
	    // Use the primitive triples as the seed to also find the
	    // non-primitive perimeters with the same ratio.
	    let mut np = p;
	    while np <= limit {
		insert_singular(np, &mut singulars, &mut duplicates);
		np += p;
	    }
	}
    }
    // The final answer is just the number of elements in the
    // singulars set.
    singulars.len() as u32
}

// This function first inserts values in the singular_set. When it
// encounters the number again, it moves it to the duplicates_set.
fn insert_singular(n: u32, singular_set: &mut HashSet<u32>, duplicates_set: &mut HashSet<u32>) {
    if duplicates_set.contains(&n) { return }
    if singular_set.contains(&n) {
	duplicates_set.insert(n);
	singular_set.remove(&n);
    } else {
	singular_set.insert(n);
    }
}
