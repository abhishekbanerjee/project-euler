fn main() {
    let num = triangular_number_with_divisors(500);
    println!("{}", num);
}

fn triangular_number_with_divisors(d: u32) -> u64 {
    let mut seed = 2u64;
    loop {
	let num = if seed % 2 == 0 { seed/2*(seed+1) } else { (seed+1)/2*seed };
	let divisors = divisors_count(num);
	if divisors > d {
	    return num;
	}
	seed += 1;
    }
}

fn divisors_count(n: u64) -> u32 {
    let root = (n as f64).sqrt().floor() as u64;
    let mut divisors_count = 2u32;
    for d in 2..root {
	if n % d == 0 {
	    divisors_count += 2;
	}
    }
    if n % root == 0 {
	divisors_count += 1;
    }
    divisors_count
}
