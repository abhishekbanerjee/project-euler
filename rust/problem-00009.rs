fn main() {
    let special = special_pythagorean_triplet(1_000);
    println!("{}", special);
}

fn special_pythagorean_triplet(n: u32) -> u64 {
    for a in 1..=n {
	for b in 1..=a {
	    match root(a*a + b*b) {
		None => {},
		Some(c) => {
		    if a + b + c == n {
			return (a as u64) * (b as u64) * (c as u64);
		    }
		},
	    }
	}
    }
    return 0;
}

fn root(n: u32) -> Option<u32> {
    let r = (n as f64).sqrt().floor() as u32;
    if r*r == n {
	Some(r)
    } else {
	None
    }
}
