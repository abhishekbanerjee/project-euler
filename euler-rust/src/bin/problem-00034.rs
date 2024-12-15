fn main() {
    let sum = sum_digit_factorials();
    println!("{}", sum);
}

fn sum_digit_factorials() -> u32 {
    let mut sum = 0u32;
    let mut digit_factorials = vec![1u32; 10];
    // Compute factorials of the digits a-priori.
    for i in 1..10 {
	digit_factorials[i] = digit_factorials[i-1] * (i as u32);
    }
    // 9! * d < 10 ^ d for d > number of digits in 9!, so this is a
    // reasonable upper bound.
    let limit = digit_factorials[9] * (digit_factorials[9].to_string().len() as u32 + 1);
    for n in 3..limit {
	let mut f = 0u32;
	for digit in split_digits(n).iter() {
	    f += digit_factorials[*digit as usize];
	}
	if f == n {
	    sum += n;
	}
    }
    sum
}

fn split_digits(n: u32) -> Vec<u8> {
    let mut m = n.clone();
    let mut digits = Vec::new();
    while m != 0 {
	digits.push((m % 10) as u8);
	m /= 10;
    }
    digits
}
