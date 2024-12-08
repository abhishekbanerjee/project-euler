fn main() {
    let letter_counts = num_letter_counts(1000);
    println!("{}", letter_counts);
}

fn num_letter_counts(n: u32) -> u32 {
    let mut counts = 0u32;
    assert!(n <= 1000);
    for i in 1..=n {
	counts += letter_count(i);
    }
    counts
}

fn letter_count(n: u32) -> u32 {
    // Special case 1000.
    if n == 1000u32 {
	return 11; /* "One thousand" */
    }
    let mut count = 0u32;
    // Handle the hundreds place first.
    if n/100 > 0 {
	count += units_and_teens(n/100) + 7 /* "hundred" */;
    }
    if n % 100 != 0 {
	// If the hundreds place exists, we need to add "and".
	if count > 0 {
	    count += 3 /* "and" */;
	}
	// Handle the tens and units place.
	let m = n % 100;
	// The teens or below case.
	if m < 20 {
	    count += units_and_teens(m);
	} else {
	    // Tens and units separately.
	    count += tens(m/10);
	    if m%10 != 0 {
		count += units_and_teens(m%10);
	    }
	}
    }
    count
}

fn units_and_teens(n: u32) -> u32 {
    match n {
	1 => 3 /* "one" */,
	2 => 3 /* "two" */,
	3 => 5 /* "three" */,
	4 => 4 /* "four" */,
	5 => 4 /* "five" */,
	6 => 3 /* "six" */,
	7 => 5 /* "seven" */,
	8 => 5 /* "eight" */,
	9 => 4 /* "nine" */,
	10 => 3 /* "ten" */,
	11 => 6 /* "eleven" */,
	12 => 6 /* "twelve" */,
	13 => 8 /* "thirteen" */,
	14 => 8 /* "fourteen" */,
	15 => 7 /* "fifteen" */,
	16 => 7 /* "sixteen" */,
	17 => 9 /* "seventeen" */,
	18 => 8 /* "eighteen" */,
	19 => 8 /* "nineteen" */,
	_ => panic!("Out of range!"),
    }
}

fn tens(n: u32) -> u32 {
    match n {
	2 => 6 /* "twenty" */,
	3 => 6 /* "thirty" */,
	4 => 5 /* "forty" */,
	5 => 5 /* "fifty" */,
	6 => 5 /* "sixty" */,
	7 => 7 /* "seventy" */,
	8 => 6 /* "eighty" */,
	9 => 6 /* "ninety" */,
	_ => panic!("Out of range!"),
    }
}
