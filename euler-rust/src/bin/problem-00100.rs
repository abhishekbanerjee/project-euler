fn main() {
    let count = blue_disks(10u64.pow(12u32));
    println!("{}", count);
}

// Let t be total disks and b be blue disks. Then we want b*(b - 1) /
// t*(t - 1) = 1/2, which simplifies to the equation: (2t - 1)^2 -
// 2(2b - 1)^2 = -1. By substituting 2t-1 = x and 2b-1=y, we get the
// negative Pell's equation: x^2 - 2y^2 = -1. The solutions to this
// equation are the numerator and denominator respectively of the odd
// values of the convergent fraction series of sqrt(2).
//
// We use the method from Problem 57 to generate the convergent
// fractions of sqrt(2).
fn blue_disks(limit: u64) -> u64 {
    let mut numerator_part = 1u64;
    let mut denominator = 2u64;
    let mut idx = 2u8;
    loop {
	// Only taking the odd terms in the sequence.
	if idx % 2 == 1 {
	    let x = numerator_part + denominator;
	    let y = denominator;
	    // Check that x=2t-1 and y=2b-1 have solutions (x and y
	    // are odd) and that t > limit.
	    if x % 2 == 1 && y % 2 == 1 && (x + 1) / 2 > limit {
		return (y + 1) / 2
	    }
	}
	let new_denominator = numerator_part + 2 * denominator;
	numerator_part = denominator;
	denominator = new_denominator;
	idx += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blue_disks() {
	assert_eq!(blue_disks(100), 85);
    }
}
