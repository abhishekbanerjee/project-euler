use euler_rust::utils::nums;

fn main() {
    let sum = almost_equilateral_sum(1_000_000_000);
    println!("{}", sum);
}

// Brute force. Check triangles of the form (a, a, a+1) and (a, a,
// a-1) for Heron's formula to produce an integer area.
fn almost_equilateral_sum(limit: u64) -> u64 {
    let mut perimeter_sum = 0u64;
    for a in 2..(limit/3) {
	let p1 = 3 * a - 1;
	if nums::is_perfect_square(p1 * (a+1)) { perimeter_sum += p1; }
	let p2 = 3 * a + 1;
	if nums::is_perfect_square(p2 * (a-1)) { perimeter_sum += p2; }
    }
    perimeter_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_almost_equilateral_sum() {
	// The only such triangle with perimeter at most 20 is the
	// 5-5-6 triangle.
	assert_eq!(almost_equilateral_sum(20), 16);
    }
}
