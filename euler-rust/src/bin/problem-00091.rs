use euler_rust::utils::nums;
use std::cmp;

fn main() {
    let triangles = right_triangles(50);
    println!("{}", triangles);
}

fn right_triangles(limit: u32) -> u32 {
    // There are 3 triangles for every integer 1 <= a, b <=
    // limit. First triangle with vertices at (a, 0) and (0,
    // b). Second with (a, 0) and (a, b). Third with (a, b) and (0,
    // b).
    let mut triangles = 3 * limit * limit;

    // For triangles with the right angle at a vertex (x1, y1) with 1
    // <= x1, y1 <= limit, we consider x2 = x1 + k*y1 for some k, and
    // then solve for y2 which makes sure that the angle is a right
    // angle (product of slopes is -1). Setting the bounds of 0 <= x2,
    // y2 <= limit gives us bounds on k, and then we choose values of
    // k which make x2, y2 integers.
    for x1 in 1..=limit {
        for y1 in 1..=limit {
            let g = nums::gcd(x1, y1);
            let max = cmp::min(x1 * g / y1, (limit - y1) * g / x1) as i32;
            let min = -1 * cmp::min(y1 * g / x1, (limit - x1) * g / y1) as i32;
            triangles += cmp::max(0, max - min) as u32;
        }
    }

    triangles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_triangles() {
	assert_eq!(right_triangles(2), 14);
    }
}
