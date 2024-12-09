fn main() {
    let sum = amicable_numbers_sum(10_000);
    println!("{}", sum);
}

fn amicable_numbers_sum(limit: u32) -> u64 {
    let mut amicable_sum = 0u64;
    for n in 2..limit {
	let d = euler_rust::divisors_sum(n);
	// We eliminate all numbers for which d(n) > n, to prevent
	// double counting. We also eliminate d(n) = n because
	// amicable pairs are distinct by definition.
	if d >= n {
	    continue;
	}
	if euler_rust::divisors_sum(d) == n {
	    amicable_sum += (d + n) as u64;
	}
    }
    amicable_sum
}
