fn main() {
    let max = max_right_triangles(1000);
    println!("{}", max);
}

fn max_right_triangles(limit: u32) -> u32 {
    let mut max_p = 0u32;
    let mut max = 0u32;
    // Loop over candidate permiter values.
    for p in 4..=limit {
	let mut right_triangles = 0u32;
	// Loop over a.
	for a in 1..p/2 {
	    let a_squared = a*a;
	    // Loop over b > a.
	    for b in a+1..p/2 {
		let c = p - (a + b);
		// Invalid triangle.
		if a + b <= c { continue; }
		// Hypotenuse too small and will only get smaller.
		if c <= b { break; }
		let b_squared = b*b;
		let c_squared = c*c;
		// Check pythagoras theorem. If hypotenuse side of the
		// equation is smaller, it will only get smaller, so
		// we should break.
		if a_squared + b_squared >= c_squared {
		    if a_squared + b_squared == c_squared {
			right_triangles += 1;
		    }
		    break;
		}
	    }
	}
	if right_triangles > max {
	    max_p = p;
	    max = right_triangles;
	}
    }
    max_p
}
