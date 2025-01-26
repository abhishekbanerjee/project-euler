use euler_rust::utils::nums;

fn main() {
    let area = rectangles_count_closest_to(2_000_000);
    println!("{}", area);
}

// The key observation here is that an m x n grid would contain t(m) *
// t(n) rectangles, where t(k) is the k-th triangular number (or the
// sum of natural numbers from 1 through k, inclusive). The rest of
// the problem is setting the appropriate bounds and careful choosing
// to find the right numbers.
fn rectangles_count_closest_to(target: u32) -> u32 {
    // An upper bound on the first side is the integer closest to the
    // reverse triangular formula of the target.
    let a_bound = reverse_triangle(target);
    let mut best_area = 0u32;
    let mut best_gap = target;
    for a in (nums::int_square_root(a_bound)..=a_bound).rev() {
	let a_triangle = a * (a + 1) / 2;
	// Similarly, choose the second number to make up the rest of
	// the product.
	let b = reverse_triangle(target / a_triangle);
	let b_triangle = b * (b + 1) / 2;
	// Since we round down in the reverse_triangle function, we
	// consider answers with b and b+1 to try and approach the
	// target from both sides.
	let gap_1 = gap(a_triangle * b_triangle, target);
	let gap_2 = gap(a_triangle * (b_triangle + b + 1), target);
	if gap_1 < best_gap {
	    best_area = a * b;
	    best_gap = gap_1;
	}
	if gap_2 < best_gap {
	    best_area = a * (b + 1);
	    best_gap = gap_2;
	}
    }
    best_area
}

fn reverse_triangle(n: u32) -> u32 {
    (nums::int_square_root(8 * n + 1) - 1) / 2
}

fn gap(m: u32, n: u32) -> u32 {
    (m as i32 - n as i32).abs() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangles_count_closest_to() {
	assert_eq!(rectangles_count_closest_to(18), 6);
    }
}
