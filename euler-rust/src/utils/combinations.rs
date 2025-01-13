pub fn combinations<T: Clone>(source: &[T], length: usize) -> Vec<Vec<T>> {
    let mut seed: Vec<T> = Vec::new();
    let mut combos: Vec<Vec<T>> = Vec::new();
    combinations_recursive(source, length, 0, &mut seed, &mut combos);
    combos
}

fn combinations_recursive<T: Clone>(source: &[T], length: usize, idx: usize, current: &mut Vec<T>, combos: &mut Vec<Vec<T>>) {
    if source.len() - idx < length - current.len() { return; }
    if current.len() == length {
	combos.push(current.clone());
	return;
    }
    combinations_recursive(source, length, idx+1, current, combos);
    current.push(source[idx].clone());
    combinations_recursive(source, length, idx+1, current, combos);
    current.pop();
}
