fn main() {
    // We just use the formula for the sum of squares and the sum of
    // numbers to compute the sums quickly and subtract them. ¯\_(ツ)_/¯
    let sq_sum = square_sum(100);
    let sum_sq = sum_squares(100);
    let difference = sq_sum - sum_sq;
    println!("{}", difference);
}

fn sum_squares(n: u64) -> u64 {
    return (n * (n+1) * (2*n+1))/6;
}

fn square_sum(n: u64) -> u64 {
    let sum = (n * (n+1))/2;
    return sum*sum;
}
