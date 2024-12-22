use euler_rust::utils::nums;

fn main() {
    let diff = smallest_pentagonal_difference();
    println!("{}", diff);
}

// Loop over the pentagonal numbers. For each new number, loop over
// all the previous pentagonal numbers and check the sum and
// difference for pentagonal-ity. The first pair found is our answer.
fn smallest_pentagonal_difference() -> u64 {
    let mut pentagonal_numbers = Vec::new();
    let mut p = 1u64;
    let mut diff = 4u64;
    loop {
	for q in pentagonal_numbers.iter() {
	    if nums::is_pentagonal(p-q) && nums::is_pentagonal(p+q) {
		return p-q
	    }
	}
	pentagonal_numbers.push(p);
	p += diff;
	diff += 3;
    }
}
