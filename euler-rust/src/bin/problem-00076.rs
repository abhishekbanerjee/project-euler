fn main() {
    let num = partition_but_one(100);
    println!("{}", num);
}

// We want to calculate one less than the integer partition
// number. There is no known closed form expression for partition(n),
// but it can be calculated recursively from previously known
// values. We use the formula detailed in
// https://en.wikipedia.org/wiki/Pentagonal_number_theorem#Relation_with_partitions,
// with memoization to store previous results.
fn partition_but_one(n: usize) -> u32 {
    let pentagonals = generate_gen_pentagonal(n);
    let mut partitions: Vec<i32> = vec![1]; // partition(0) = 1
    for m in 1..=n {
	let mut p = 0i32;
	let mut sign = 1i32;
	for idx in (0..pentagonals.len()).step_by(2) {
	    if pentagonals[idx] > m { break; }
	    p += sign * partitions[m - pentagonals[idx]];
	    if pentagonals[idx+1] > m { break; }
	    p += sign * partitions[m - pentagonals[idx+1]];
	    sign *= -1;
	}
	partitions.push(p);
    }
    // One less because the partition number includes the case with a
    // single partition with the number itself, but we want at least 2
    // parts.
    partitions[n] as u32 - 1
}

// We generate and store generalized pentagonal numbers for easy
// lookup later.
fn generate_gen_pentagonal(n: usize) -> Vec<usize> {
    let mut pentagonals = Vec::new();
    let mut k = 1usize;
    loop {
	pentagonals.push(k * (3 * k - 1) / 2);
	pentagonals.push(k * (3 * k + 1) / 2);
	if pentagonals[2 * k - 1] >= n { break; }
	k += 1;
    }
    pentagonals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_but_one() {
	assert_eq!(partition_but_one(5), 6);
    }
}
