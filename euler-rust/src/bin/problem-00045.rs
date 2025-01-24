use euler_rust::utils::nums;

fn main() {
    let n = hexagonal_also_triangular_pentagonal_after(143);
    println!("{}", n);
}

// We find the hexagonal number which is also triangular and
// pentagonal after the hex_idx-th hexagonal number. We do that by
// looping over the hexagonal numbers using the formula H_{n+1} -
// H_{n} = 4n+1. At each step, we will check if the number is also
// triangular and pentagonal.
fn hexagonal_also_triangular_pentagonal_after(hex_idx: u64) -> u64 {
    // Start of the hexagonal number to check and the diff to compute
    // the next one.
    let mut next_hex = (hex_idx + 1) * (2 * (hex_idx + 1) - 1);
    let mut diff = 4 * (hex_idx + 1) + 1;
    loop {
	if nums::is_triangular(next_hex) && nums::is_pentagonal(next_hex) {
	    return next_hex
	}
	next_hex += diff;
	diff += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexagonal_also_triangular_pentagonal_after() {
	assert_eq!(hexagonal_also_triangular_pentagonal_after(1), 40755);
    }
}
