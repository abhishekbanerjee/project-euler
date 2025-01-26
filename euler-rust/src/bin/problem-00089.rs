use euler_rust::utils::files;

fn main() {
    let chars = roman_chars_saved_file("resources/0089_roman.txt");
    println!("{}", chars);
}

fn roman_chars_saved_file(file_name: &str) -> u32 {
    files::read_file(file_name).lines().map(|r| roman_chars_saved(r)).sum()
}

// Difference of the characters of the raw and canonicalized
// string. To find the canonicalized string, convert the Roman number
// to u32 and then back to a canonicalized Roman number.
fn roman_chars_saved(numeral: &str) -> u32 {
    let non_canonical = numeral.chars().count() as u32;
    let num = roman_to_int(numeral);
    let canonical = int_to_roman(num).chars().count() as u32;
    non_canonical - canonical
}

fn roman_to_int(numeral: &str) -> u32 {
    let mut num = 0u32;
    let mut prev = u32::MAX;
    for c in numeral.chars() {
	let current = roman_char_to_int(c);
	num += current - if prev < current { 2 * prev } else { 0 };
	prev = current;
    }
    num
}

fn roman_char_to_int(roman: char) -> u32 {
    match roman {
	'I' => 1,
	'V' => 5,
	'X' => 10,
	'L' => 50,
	'C' => 100,
	'D' => 500,
	'M' => 1000,
	_ => panic!("{} not a valid Roman numeral!", roman),
    }
}

fn int_to_roman(mut n: u32) -> String {
    let mut roman = String::from("");
    while n >= 1000 {
	roman.push('M');
	n -= 1000;
    }
    let (h, t, u) = (n / 100, (n % 100) / 10, n % 10);
    roman.push_str(digit_to_roman(h, 'M', 'D', 'C').as_str());
    roman.push_str(digit_to_roman(t, 'C', 'L', 'X').as_str());
    roman.push_str(digit_to_roman(u, 'X', 'V', 'I').as_str());
    roman
}

fn digit_to_roman(d: u32, ten: char, five: char, unit: char) -> String {
    let mut roman = String::from("");
    match d {
	0..=3 => for _ in 0..d { roman.push(unit); },
	4 => { roman.push(unit); roman.push(five); },
	5..=8 => {
	    roman.push(five);
	    for _ in 0..(d-5) { roman.push(unit); }
	},
	9 => { roman.push(unit); roman.push(ten); },
	_ => panic!("Impossible!"),
    }
    roman
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_chars_saved() {
	assert_eq!(roman_chars_saved("XIIIIII"), 4);
    }
}
