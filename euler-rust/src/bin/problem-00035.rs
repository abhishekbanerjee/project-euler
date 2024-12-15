use euler_rust::utils::primes::Primes;

fn main() {
    let count = circular_primes(1_000_000);
    println!("{}", count);
}

fn circular_primes(limit: usize) -> usize {
    let mut count = 0usize;
    let mut primes = Primes::up_to(limit+1);
    for prime in primes.all_primes_below(limit).iter() {
	let mut is_rotated_prime = true;
	for rotated in rotations(*prime).iter() {
	    if !primes.is_prime(*rotated) {
		is_rotated_prime = false;
		break;
	    }
	}
	if is_rotated_prime {
	    count += 1;
	}
    }
    count
}

// Get all the rotated numbers for a given number except the number
// itself. Since we're only considering primes in the first place, we
// don't want to check the number itself again.
fn rotations(n: usize) -> Vec<usize> {
    let mut m = n.clone();
    let l = n.to_string().len();
    let mut rotated = Vec::new();
    for _ in 1..l {
	m = 10usize.pow((l-1) as u32) * (m%10) + m/10;
	rotated.push(m);
    }
    rotated
}
