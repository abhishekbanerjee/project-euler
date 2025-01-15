fn main() {
    let difference = sum_square_difference(100);
    println!("{}", difference);
}

// We just use the formula for the sum of squares and the sum of
// numbers to compute the sums quickly and subtract them.
fn sum_square_difference(n: u64) -> u64 {
    square_sum(n) - sum_squares(n)
}

fn sum_squares(n: u64) -> u64 {
    (n * (n+1) * (2*n+1))/6
}

fn square_sum(n: u64) -> u64 {
    let sum = (n * (n+1))/2;
    sum*sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_square_difference() {
	assert_eq!(sum_square_difference(10), 2640);
    }
}
