use euler_rust::utils::files;

fn main() {
    let largest = largest_exponential("resources/0099_base_exp.txt");
    println!("{}", largest);
}

// Loop over the numbers in the file, keeping track of the largest
// using a custom comparison function detailed below.
fn largest_exponential(file_path: &str) -> u32 {
    let mut largest_idx = 0u32;
    let mut largest_num = (0u32, 1u32);
    let mut idx = 1u32;
    for line in files::read_file(file_path).lines() {
	let nums = line.split(",").collect::<Vec<_>>();
	let num = (nums[0].parse::<u32>().unwrap(), nums[1].parse::<u32>().unwrap());
	if is_greater(num, largest_num) {
	    largest_num = num;
	    largest_idx = idx;
	}
	idx += 1;
    }
    largest_idx
}

fn is_greater(a: (u32, u32), b: (u32, u32)) -> bool {
    // If both base and exponent comparisons agree in one direction,
    // we can short-circuit.
    if a.0 >= b.0 && a.1 >= b.1 { return true }
    if b.0 >= a.0 && b.1 >= a.1 { return false }
    // To compare a^e and b^f, compute a^(e/f) (as a floating point),
    // and compare to b. Since we only care about a greater than
    // comparison, we don't care too deeply about decimal precision.
    (a.0 as f64).powf(a.1 as f64 / b.1 as f64) > b.0 as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_greater() {
	assert!(!is_greater((2, 11), (3, 7)));
	assert!(is_greater((632382, 518061), (519432, 525806)));
    }
}
