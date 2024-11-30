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
	multiple = lcm(multiple, factor);
    }
    return multiple;
}

// We use the formula: LCM(a, b) = a * b / GCD(a, b), where GCD is the
// greatest common divisor.
fn lcm(a: u64, b: u64) -> u64 {
    return a*b/gcd(a,b);
}

// We use the resursive remainder method to find GCD. The numbers are
// not too big so the recursion depth is shallow. This can be
// optimized to remove recursion.
fn gcd(a: u64, b: u64) -> u64 {
    if a > b {
	return gcd(b, a);
    }
    if a == 0 {
	return b;
    }
    return gcd(b % a, a);
}
