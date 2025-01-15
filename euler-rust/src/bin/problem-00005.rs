use euler_rust::utils::nums;

fn main() {
    let smallest = smallest_multiple(20);
    println!("{}", smallest);
}

// The smallest multiple of a group of integers is also called the
// "lowest common multiple" or LCM. This is distributive, so LCM(a, b,
// c) = LCM(LCM(a, b), c).
fn smallest_multiple(n: u64) -> u64 {
    let mut multiple = 1u64;
    for factor in 1..=n {
	multiple = nums::lcm(multiple, factor);
    }
    multiple
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_multiple() {
	assert_eq!(smallest_multiple(10), 2520);
    }
}
