use euler_rust::utils::nums;

fn main() {
    let product = champernowne_product(vec![1u32, 10, 100, 1_000, 10_000, 100_000, 1_000_000]);
    println!("{}", product);
}

fn champernowne_product(indices: Vec<u32>) -> u32 {
    let mut product = 1u32;
    for index in indices.iter() {
	product *= champernowne_number(index.clone());
    }
    product
}

// Returns the index-th (1 based, like the question) digit in the
// Champernowne decimal expansion.
fn champernowne_number(mut index: u32) -> u32 {
    let mut d = 1u32;
    loop {
	// Count the number of d-digited numbers there are and what is
	// their length in the Champernowne decimal expansion, to see
	// if our index fits in the d-digited numbers.
	let limit = (10u32.pow(d-1) * 9) * d;
	if index > limit {
	    index -= limit;
	    d += 1;
	    continue;
	}
	// Find the right d-digited number pointed by the index, and
	// it's position in that number.
	let target = 10u32.pow(d-1) + (index - 1) / d;
	let pos = (index-1) % d;
	return nums::nth_digit(target, pos as usize);
    }
}
