use std::collections::HashSet;

fn main() {
    let sum = product_sum_sum(12_000);
    println!("{}", sum);
}

fn product_sum_sum(limit: usize) -> u32 {
    // Build a cache of k -> minimal product-sum number for k.
    let mut minimals = vec![u32::MAX; limit  + 1];
    // Start off the process with a product of 1, sum of 0, and a
    // starting factor of 2.
    build_product_sums(1, 0, 0, 2, &mut minimals);
    // Skip first two elements (k = 0 and 1 are undefined), remove
    // duplicates and sum it up.
    minimals.into_iter().skip(2).collect::<HashSet<u32>>().into_iter().sum::<u32>()
}

fn build_product_sums(n: u32, factor_sum: u32, factors: u32, first: u32, cache: &mut Vec<u32>) {
    // If we add (n - factor_sum) 1s to the other factors collected so
    // far, we would have n be a product-sum candidate for a set of
    // this size.
    let k = (n - factor_sum + factors) as usize;
    if k >= cache.len() {
	return
    }
    // Make sure that we have at least 1 non-trivial factor, and
    // update the minimal value if needed.
    if factors > 1 && n < cache[k] {
	cache[k] = n;
    }
    // Iterate over other possible factors. We choose our limit by
    // noting that for any k, 2 * k is a product-sum candidate (by
    // choosing factors, 2, k and (k-2) 1s), and thus a bound for the
    // minimal one..
    for i in first..=(2 * cache.len() as u32 / n) {
	build_product_sums(n * i, factor_sum + i, factors + 1, i, cache);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_sum_sum() {
	assert_eq!(product_sum_sum(6), 30);
    }
}
