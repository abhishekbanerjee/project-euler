fn main() {
    let smallest = smallest_multiple(20);
    println!("{}", smallest);
}

fn smallest_multiple(n: u64) -> u64 {
    let mut multiple = 1u64;
    for factor in 1..=n {
	multiple = lcm(multiple, factor);
    }
    return multiple;
}

fn lcm(a: u64, b: u64) -> u64 {
    return a*b/gcd(a,b);
}

fn gcd(a: u64, b: u64) -> u64 {
    if a > b {
	return gcd(b, a);
    }
    if a == 0 {
	return b;
    }
    return gcd(b % a, a);
}
