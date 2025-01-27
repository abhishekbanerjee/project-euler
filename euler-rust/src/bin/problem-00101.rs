fn main() {
    let sum = bad_optimal_polynomial_sum(&[1, -1, 1, -1 ,1, -1, 1, -1, 1, -1, 1]);
    println!("{}", sum);
}

// We use Newton's divided differences method to find the successive
// interpolation polynomials, and evaluate them at the next term to
// find the first incorrect fit.
fn bad_optimal_polynomial_sum(coefficients: &[i64]) -> i64 {
    let mut sum = 0;
    let evaluations = (1..=coefficients.len())
	.map(|n| evaluate_polynomial(coefficients, n as i64))
	.collect::<Vec<_>>();
    let mut divided_diffs = evaluations.clone();
    let mut leading_diff: Vec<i64> = Vec::new();
    for diff in 1..coefficients.len() {
	leading_diff.push(divided_diffs[0]);
	let mut term = 0i64;
	let mut product = 1i64;
	for (i, &c) in leading_diff.iter().enumerate() {
	    term += c * product;
	    product *= (diff - i) as i64;
	}
	if term != evaluations[diff] { sum += term; }
	divided_diffs = divided_diffs
	    .iter()
	    .skip(1)
	    .zip(divided_diffs.iter())
	    .map(|(&n1, &n2)| (n1 - n2) / diff as i64)
	    .collect();
    }
    sum
}

fn evaluate_polynomial(coefficients: &[i64], x: i64) -> i64 {
    let mut x_pow = 1i64;
    let mut y = 0;
    for &a in coefficients.iter() {
	y += x_pow * a;
	x_pow *= x;
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_optimal_polynomial_sum() {
	assert_eq!(bad_optimal_polynomial_sum(&[0, 0, 0, 1]), 74);
    }
}
