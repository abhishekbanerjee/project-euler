use euler_rust::utils::combinations;
use euler_rust::utils::parse;
use num_rational::Ratio;
use num_traits::identities::Zero;
use std::collections::HashSet;

fn main() {
    let longest = longest_arithmetic_series(4);
    println!("{}", longest);
}

// For all combinations of four digits, collect all results of all
// possible expressions with the four operators, and choose the one
// that contains the most consecutive numbers starting at 1.
fn longest_arithmetic_series(num_digits: usize) -> u32 {
    let mut longest_series = 0u32;
    let mut longest_series_length = 0u32;
    let digits: Vec<u8> = (0u8..=9).collect();
    for combination in combinations::combinations(digits.as_slice(), num_digits).iter() {
	let series: HashSet<u32> = all_arithmetics(combination.as_slice())
	    .into_iter()
	    .filter(|&r| r > Ratio::from_integer(0) && r.is_integer())
	    .map(|r| r.to_integer() as u32)
	    .collect();
	let mut series_length = 0u32;
	loop {
	    if !series.contains(&(series_length + 1)) { break; }
	    series_length += 1;
	}
	if series_length > longest_series_length {
	    longest_series_length = series_length;
	    longest_series = parse::parse_slice_as_number::<u8, u32>(combination.as_slice());
	}
    }
    longest_series
}

fn all_arithmetics(nums: &[u8]) -> HashSet<Ratio<i32>> {
    match nums.len() {
	1 => return HashSet::from([Ratio::from_integer(nums[0] as i32)]),
	2 => return ops(all_arithmetics(&nums[..1]), all_arithmetics(&nums[1..])),
	3 => {
	    let mut results: HashSet<Ratio<i32>> = ops(all_arithmetics(&[nums[0]]), all_arithmetics(&[nums[1], nums[2]]));
	    results.extend(ops(all_arithmetics(&[nums[1]]), all_arithmetics(&[nums[0], nums[2]])).iter());
	    results.extend(ops(all_arithmetics(&[nums[2]]), all_arithmetics(&[nums[0], nums[1]])).iter());
	    return results
	},
	4 => {
	    let mut results: HashSet<Ratio<i32>> = ops(all_arithmetics(&[nums[0]]), all_arithmetics(&[nums[1], nums[2], nums[3]]));
	    results.extend(ops(all_arithmetics(&[nums[1]]), all_arithmetics(&[nums[0], nums[2], nums[3]])).iter());
	    results.extend(ops(all_arithmetics(&[nums[2]]), all_arithmetics(&[nums[0], nums[1], nums[3]])).iter());
	    results.extend(ops(all_arithmetics(&[nums[3]]), all_arithmetics(&[nums[0], nums[1], nums[2]])).iter());
	    results.extend(ops(all_arithmetics(&[nums[0], nums[1]]), all_arithmetics(&[nums[2], nums[3]])).iter());
	    results.extend(ops(all_arithmetics(&[nums[0], nums[2]]), all_arithmetics(&[nums[1], nums[3]])).iter());
	    results.extend(ops(all_arithmetics(&[nums[0], nums[3]]), all_arithmetics(&[nums[1], nums[2]])).iter());
	    return results
	},
	_ => panic!("Impossible!"),
    }
}

fn ops(a: HashSet<Ratio<i32>>, b: HashSet<Ratio<i32>>) -> HashSet<Ratio<i32>> {
    let mut results: HashSet<Ratio<i32>> = HashSet::new();
    for &r1 in a.iter() {
	for &r2 in b.iter() {
	    results.extend([r1 + r2, r1 * r2, r1 - r2, r2 - r1].iter());
	    if !r1.is_zero() { results.insert(r2 / r1); }
	    if !r2.is_zero() { results.insert(r1 / r2); }
	}
    }
    results
}
