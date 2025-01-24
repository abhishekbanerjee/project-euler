use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;

fn main() {
    let digits = self_power_last_ten(1000);
    println!("{}", digits);
}

fn self_power_last_ten(limit: u32) -> u64 {
    // Use the big-int library for the large exponentiation. Sum up
    // the series and extract the last 10 digits by taking the
    // remainder from a division by 10^10.
    ((1..=limit).map(|n| BigUint::from(n).pow(n)).sum::<BigUint>()
     % BigUint::from(10u8).pow(10)).to_u64().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_power_last_ten() {
	assert_eq!(self_power_last_ten(10), 405071317);
    }
}
