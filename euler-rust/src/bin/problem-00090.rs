fn main() {
    let combinations = distinct_cube_arrangements(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 6);
    println!("{}", combinations);
}

// Build all possible 6-sided die combinations. Take all pairs of dies
// and see which are valid, and sum up the count.
fn distinct_cube_arrangements(cube_digits: &[u8], cube_len: usize) -> u32 {
    let square_digits: Vec<(u8, u8)> = (1..=9).map(|i| ((i * i) / 10, (i * i) % 10)).collect();
    let combos = combinations(cube_digits, cube_len);
    let mut count = 0u32;
    for i in 0..combos.len() {
	for j in (i+1)..combos.len() {
	    count += valid(square_digits.as_slice(), combos[i].as_slice(), combos[j].as_slice()) as u32;
	}
    }
    count
}

fn valid(to_find: &[(u8, u8)], dice1: &[u8], dice2: &[u8]) -> bool {
    let mut result = true;
    for &(n1, n2) in to_find.iter() {
	if !(contains(n1, dice1) && contains(n2, dice2) || contains(n1, dice2) && contains(n2, dice1)) {
	    result = false;
	    break;
	}
    }
    result
}

fn contains(n: u8, set: &[u8]) -> bool {
    if n == 6 || n == 9 {
	set.contains(&6u8) || set.contains(&9u8)
    } else {
	set.contains(&n)
    }
}

fn combinations(base: &[u8], target_size: usize) -> Vec<Vec<u8>> {
    let mut combos_seed: Vec<u8> = Vec::new();
    let mut combos: Vec<Vec<u8>> = Vec::new();
    combinations_recursive(base, target_size, 0, &mut combos_seed, &mut combos);
    combos
}

fn combinations_recursive(base: &[u8], target_size: usize, idx: usize, combo: &mut Vec<u8>, combos: &mut Vec<Vec<u8>>) {
    if combo.len() == target_size {
	combos.push(combo.clone());
	return
    }
    if base.len() - idx < target_size - combo.len() {
	return
    }
    combinations_recursive(base, target_size, idx+1, combo, combos);
    combo.push(base[idx]);
    combinations_recursive(base, target_size, idx+1, combo, combos);
    combo.pop();
}
