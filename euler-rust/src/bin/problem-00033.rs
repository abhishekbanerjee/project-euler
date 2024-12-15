use euler_rust::utils::nums;

fn main() {
    let denominator = digit_cancelling_fractions();
    println!("{}", denominator);
}

// We iterate over the canceled digit 'c', numerator digit 'n' and
// denominator digit 'd' (with n < d because of our fraction < 1
// constraint). The 2-digit numerators are 10*n + c or 10*c + n and
// the 2-digit denominators are 10*d + c or 10*c + d. To avoid clunky
// division, we compute candidate LHS and RHS for our comparison
// equations (LHS = 2-digit numerator * d and RHS = 2-digit
// denominator * d) and compare those to see when LHS = RHS.
fn digit_cancelling_fractions() -> u32 {
    let mut numerators = 1u32;
    let mut denominators = 1u32;
    for c in 1..=9 {
	for n in 1..9 {
	    for d in (n+1)..9 {
		let l1 = (10*n + c) * d;
		let l2 = (10*c + n) * d;
		let r1 = (10*d + c) * n;
		let r2 = (10*c + d) * n;
		if l1 == r1 || l1 == r2 || l2 == r1 || l2 == r2 {
		    numerators *= n;
		    denominators *= d;
		}
	    }
	}
    }
    denominators/nums::gcd(numerators,denominators)
}
