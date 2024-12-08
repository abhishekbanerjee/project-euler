fn main() {
    let sundays = count_sundays((1901u32, 01u32), (2000u32, 12u32));
    println!("{}", sundays);
}

fn count_sundays(start_date: (u32, u32), end_date: (u32, u32)) -> u32 {
    let mut sundays = 0u32;
    let mut year = 1900u32;
    let mut month = 01u32;
    let mut current_day = 01u32;
    while year <= end_date.0 && month <= end_date.1 {
	if current_day == 0 && year >= start_date.0 {
	    sundays += 1
	}
	current_day = (current_day + days_per_month(year, month)) % 7;
	if month == 12 {
	    year += 1;
	}
	month = (month % 12) + 1;
    }
    sundays
}

fn days_per_month(year: u32, month: u32) -> u32 {
    match month {
	1 => 31 /* January */,
	2 => { /* February */
	    if year % 4 == 0 && year != 1900 { 29 } else { 28 }
	},
	3 => 31 /* March */,
	4 => 30 /* April */,
	5 => 31 /* May */,
	6 => 30 /* June */,
	7 => 31 /* July */,
	8 => 31 /* August */,
	9 => 30 /* September */,
	10 => 31 /* October */,
	11 => 30 /* November */,
	12 => 31 /* December */,
	_ => panic!("Out of range!"),
    }
}
