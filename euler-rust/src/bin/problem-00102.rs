use euler_rust::utils::files;

fn main() {
    let count = count_origin_within_triangles("resources/0102_triangles.txt");
    println!("{}", count);
}

fn count_origin_within_triangles(file_path: &str) -> u32 {
    let mut count = 0u32;
    for triangle_line in files::read_file(file_path).lines() {
	let co_ords = triangle_line.split(",").into_iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
	if is_origin_in_triangle((co_ords[0], co_ords[1]), (co_ords[2], co_ords[3]), (co_ords[4], co_ords[5])) { count += 1; }
    }
    count
}

// To test if the origin lies in the triangle, we compute the sum of
// the areas of the triangles formed by the origin and pairs of the
// points. If the origin is in the triangle, then this sum would be
// equal to the area of the triangle formed by the three points.
fn is_origin_in_triangle(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> bool {
    let o = (0i32, 0i32);
    area(p1, p2, p3) == area(o, p1, p2) + area(o, p2, p3) + area(o, p3, p1)
}

fn area((x1, y1): (i32, i32), (x2, y2): (i32, i32), (x3, y3): (i32, i32)) -> f32 {
    (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs() as f32 / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_origin_in_triangle() {
	assert!(is_origin_in_triangle((-340, 495), (-153, -910), (835, -947)));
	assert!(!is_origin_in_triangle((-175, 41), (-421, -714), (574, -645)));
    }
}
