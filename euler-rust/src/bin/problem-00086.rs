use euler_rust::utils::nums;

fn main() {
    let limit = limit_for_cuboids_numbering(1_000_000);
    println!("{}", limit);
}

// To enforce uniqueness, we enforce a >= b >= c in our cuboids, and
// increment a by 1 in each step. We notice that a and b+c must form
// the two sides of a right triangle.
fn limit_for_cuboids_numbering(num: u32) -> u32 {
    let mut count = 0u32;
    let mut a = 1u32;
    loop {
	let a_squared = a * a;
	// The b+c sum can be as large as 2*a for the uniqueness condition.
	for b_c in 1..=2*a {
	    // Check Pythagoras Theorm.
	    if !nums::is_perfect_square(b_c * b_c + a_squared) { continue; }
	    // Increment count appropriately. There are two cases,
	    // depending on whether b+c <= a.
	    count += if b_c <= a { b_c / 2 } else { (2 * a - b_c) / 2 + 1 };
	}
	if count > num { break; }
	a += 1;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_for_cuboids_numbering() {
	assert_eq!(limit_for_cuboids_numbering(2000), 100);
    }
}
