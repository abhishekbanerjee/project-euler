use num_bigint::BigInt;

fn main() {
    let num = partition_divisible_by(1_000_000);
    println!("{}", num);
}

// We calculate partitions of numbers, using the exact same method as
// Problem 76, until our divisibility condition is satisfied. There
// are some changes, to grow the cache of generalized pentagonals on
// demand, and to handle big numbers.
fn partition_divisible_by(d: u32) -> usize {
    let mut pentagonals = Vec::new();
    generate_gen_pentagonal(&mut pentagonals);
    let mut partitions: Vec<BigInt> = vec![BigInt::from(1)];
    let mut n = 1usize;
    loop {
	if &n > pentagonals.last().unwrap() {
	    generate_gen_pentagonal(&mut pentagonals);
	}
	let mut p = BigInt::ZERO;
	let mut sign = BigInt::from(1);
	for idx in (0..pentagonals.len()).step_by(2) {
	    if pentagonals[idx] > n { break; }
	    p += &sign * &partitions[n - pentagonals[idx]];
	    if pentagonals[idx+1] > n { break; }
	    p += &sign * &partitions[n - pentagonals[idx+1]];
	    sign *= -1;
	}
	partitions.push(p.clone());
	if p % BigInt::from(d) == BigInt::ZERO { break; }
	n += 1;
    }
    n
}

// We generate and store generalized pentagonal numbers for easy
// lookup later. Generate 1000 pentagonals at a time.
fn generate_gen_pentagonal(pentagonals: &mut Vec<usize>) {
    let l = pentagonals.len();
    for k in (l/2+1)..=(l/2+500) {
	pentagonals.push(k * (3 * k - 1) / 2);
	pentagonals.push(k * (3 * k + 1) / 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_divisible_by() {
	assert_eq!(partition_divisible_by(7), 5);
    }
}
