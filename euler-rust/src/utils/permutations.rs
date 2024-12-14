pub struct Permutations<T: Ord + Clone> {
    state: Vec<T>,
    started: bool,
    is_done: bool,
}

impl<T: Ord + Clone> Permutations<T> {
    pub fn with(seed: &[T]) -> Permutations<T> {
	let mut state = seed.to_vec();
	state.sort();
	Permutations { state, started: false, is_done: false }
    }

    pub fn next_permutation(&mut self) -> Option<&[T]> {
	if self.started {
	    self.move_forward();
	} else {
	    self.started = true;
	}
	if self.is_done { None } else { Some(self.state.as_slice()) }
    }

    // Generates the next lexicographical permutation, according to
    // the algorithm described in
    // https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order.
    fn move_forward(&mut self) {
	if self.is_done {
	    return;
	}
	let l = self.state.len();
	if l == 1 {
	    self.is_done = true;
	    return
	}
	let mut i = l-2;
	loop {
	    if self.state[i] < self.state[i+1] { break; }
	    // This means we're already in decreasing order and thus
	    // at the last permutation.
	    if i == 0 {
		self.is_done = true;
		return
	    }
	    i -= 1;
	}
	let mut j = i+1;
	while j < l {
	    if self.state[j] < self.state[i] { break; }
	    j += 1;
	}
	self.state.swap(j-1, i);
	self.state[i+1..].reverse();
    }
}
