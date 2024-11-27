fn main() {
    let sum = sum_fibonaccis(4_000_000);
    println!("{}", sum);
}

fn sum_fibonaccis(n: u64) -> u64 {
    let mut sum = 0u64;
    let mut a = 1u64;
    let mut b = 2u64;
    while b <= n {
	if b % 2 == 0 {
	    sum += b;
	}
	let c = a + b;
	a = b;
	b = c;
    }
    return sum
}
