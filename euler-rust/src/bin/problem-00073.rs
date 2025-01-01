use euler_rust::utils::nums;

fn main() {
    let count = fraction_count_between_third_and_half(12_000);
    println!("{}", count);
}

// Iterate over the denominators and the relevant numerators, only
// counting the proper fractions.
fn fraction_count_between_third_and_half(limit: u32) -> u32 {
    let mut count = 0u32;
    for d in 4..=limit {
	for n in (d/3+1)..=d/2 {
	    if nums::gcd(n, d) == 1 {
		count += 1;
	    }
	}
    }
    count
}
