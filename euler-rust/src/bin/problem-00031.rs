fn main() {
    let ways = find_combinations(&[1,2,5,10,20,50,100,200], 200);
    println!("{}", ways);
}

// Dynamic programming, iterating over the denominated coins in the
// outer loop and the amounts to make up in the inner loop. This
// considers each coin only once and does not double count.
fn find_combinations(denominations: &[usize], total: usize) -> usize {
    let mut combinations = vec![0usize; total+1];
    combinations[0] = 1usize;
    for coin in denominations.iter() {
	for amount in 1..=total {
	    if *coin > amount { continue; }
	    combinations[amount] += combinations[amount-coin];
	}
    }
    combinations[total]
}
