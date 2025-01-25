use euler_rust::utils::nums;

fn main() {
    let numerator = three_seventh_left_neighbour(1_000_000);
    println!("{}", numerator);
}

// Store the target ratio as a decimal. For each denominator, choose a
// target numerator by taking the floor of the denominator times the
// target ratio.
fn three_seventh_left_neighbour(limit: u32) -> u32 {
    let target = 3.0/7.0;
    let mut closest_r = 0.0;
    let mut closest_n = 0.0;
    for d in 2..=limit {
	// Don't choose the ratio we're looking for.
	if d == 7 { continue; }
	let n = (target * (d as f64)).floor();
	// If the fraction is not in reduced form, ignore.
	if nums::gcd(n as u32, d) != 1 { continue; }
	// If the gap between the target and this ratio is smaller
	// than the smallest found so far, update.
	let r = n/(d as f64);
	if r > closest_r {
	    closest_r = r;
	    closest_n = n;
	}
    }
    closest_n as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_seventh_left_neighbour() {
	assert_eq!(three_seventh_left_neighbour(8), 2);
    }
}
