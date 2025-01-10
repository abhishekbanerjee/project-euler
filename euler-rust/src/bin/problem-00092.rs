fn main() {
    let count = square_digit_89(7);
    println!("{}", count);
}

fn square_digit_89(limit_digits: u8) -> u32 {
    let mut count = 0u32;
    // We build a cache of the terminating value for each of the
    // numbers that could be the sum of squares of a number between 1
    // and the lmiit.
    let cache = build_cache(limit_digits);
    // Total number of limit_digit numbers. Helps in a calculation
    // ahead.
    let combinations = factorial(limit_digits as u32);
    // For every way to split limit_digit balls in ten bins [basically
    // each digit permiation of limit_digit numbers].
    for split in digit_splits(limit_digits).iter() {
	// Find the sum of squares of this digit combo.
	let s = evaluate_split(split.as_slice());
	// Look this up in the prebuilt cache.
	if cache[s as usize] == 89 {
	    // If found, add the appropriate number of numbers that
	    // have this digit combination.
	    count += combinations / factorial_product(split.as_slice());
	}
    }
    count
}

fn build_cache(limit_digits: u8) -> Vec<u8> {
    let limit = 9 * 9 * (limit_digits as usize);
    let mut terminators = vec![0u8; limit+1];
    terminators[1] = 1;
    terminators[89] = 89;
    for i in 1..=limit {
	let mut m = i;
	let mut chain = Vec::new();
	while terminators[m] == 0 {
	    chain.push(m);
	    m = evaluate_num(m as u32) as usize;
	}
	for j in chain.into_iter() {
	    terminators[j] = terminators[m];
	}
    }
    terminators
}

fn digit_splits(limit_digits: u8) -> Vec<Vec<u8>> {
    let mut seed = vec![0u8; 10];
    let mut all_splits: Vec<Vec<u8>> = Vec::new();
    digit_splits_recursive(0, limit_digits, &mut seed, &mut all_splits);
    return all_splits;
}

fn digit_splits_recursive(idx: usize, limit: u8, split: &mut Vec<u8>, all_splits: &mut Vec<Vec<u8>>) {
    if idx == split.len() {
	if limit == 0  { all_splits.push(split.clone()); }
	return
    }
    for i in 0..=limit {
	split[idx] = i;
	digit_splits_recursive(idx + 1, limit - i, split, all_splits);
    }
    split[idx] = 0;
}

fn evaluate_num(mut num: u32) -> u32 {
    let mut sum = 0u32;
    while num != 0 {
	sum += (num % 10) * (num % 10);
	num /= 10;
    }
    sum
}

fn evaluate_split(nums: &[u8]) -> u32 {
    let mut sum = 0u32;
    for (i, &c) in nums.iter().enumerate() {
	sum += (c as u32) * (i * i) as u32;
    }
    sum
}

fn factorial_product(nums: &[u8]) -> u32 {
    nums.iter().map(|&i| factorial(i as u32)).product()
}

fn factorial(n: u32) -> u32 {
    (1..=n).product()
}
