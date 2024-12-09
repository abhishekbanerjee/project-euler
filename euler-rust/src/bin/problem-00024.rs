fn main() {
    let permutation = lexicographic_permutation_number(1_000_000);
    println!("{}", permutation);
}

fn lexicographic_permutation_number(n: u32) -> String {
    let mut state: Vec<u32> = (0..10).collect();
    for _ in 1..n {
	next_permutation(state.as_mut_slice());
    }
    state.into_iter().map(|i| i.to_string()).collect::<String>()
}

fn next_permutation(state: &mut [u32]) {
    let l = state.len();
    let mut i = l-2;
    loop {
	if state[i] < state[i+1] { break; }
	i -= 1;
    }
    let mut j = i+1;
    while j < l {
	if state[j] < state[i] { break; }
	j += 1;
    }
    state.swap(j-1, i);
    state[i+1..].reverse();
}

