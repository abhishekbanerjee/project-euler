fn main() {
    let modal = monopoly_most_popular(4);
    println!("{}", modal);
}

const GO: usize = 0;
const JAIL: usize = 10;
const G2J: usize = 30;
const CC: &[usize] = &[2, 17, 33];
const CH: &[usize] = &[7, 22, 36];
const NEXT_R: &[usize] = &[15, 25, 5];
const NEXT_U: &[usize] = &[12, 28, 12];

const NUM_SQUARES: usize = 40;
const DOUBLES_JAIL: usize = 3;

const EPSILON: f64 = 10e-9;

// We use a Markov-Chain to tabulate all probabilities and simulate
// the die rolls to see how the probability vector evolved. Once the
// distance between successive states is below a certain threshold, we
// stop and evaluate the final result.
fn monopoly_most_popular(die_sides: u8) -> String {
    let probabilities = build_die_probability(die_sides);
    let probability_matrix = build_probability_matrix(probabilities.as_slice());

    // Initial state. We're at square 0 (GO) with 0 doubles so far: encoded as 0.
    let mut state = vec![0.0; NUM_SQUARES * DOUBLES_JAIL];
    state[0] = 1.0;

    // Markov-Chain simulation.
    loop {
	let next_state = mult(&state, &probability_matrix);
	let mut small_enough = true;
	for i in 0..state.len() {
	    if (state[i] - next_state[i]).abs() >= EPSILON {
		small_enough = false;
		break;
	    }
	}
	state = next_state;
	if small_enough { break; }
    }

    // Collapse the state vector into probabilities for finishing at
    // particular squares (we don't care about doubles any more).
    let mut final_state = vec![(0.0, 0); NUM_SQUARES];
    for (i, &p) in state.iter().enumerate() {
	let (square, _) = decode(i);
	final_state[square].0 += p;
	final_state[square].1 = square;
    }

    // Sort the state vector in descending order and pick the first
    // three indices.
    final_state.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    format!("{:02}{:02}{:02}", final_state[0].1, final_state[1].1, final_state[2].1).to_string()
}

// Vector-matrix multiplication.
fn mult(vector: &[f64], matrix: &Vec<Vec<f64>>) -> Vec<f64> {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut result = vec![0.0; n];
    for j in 0..n {
	for i in 0..m {
	    result[j] += vector[i] * matrix[i][j];
	}
    }
    result
}

// Builds the probability matrix from the die probabilities.
fn build_probability_matrix(die_probability: &[(f64, f64)]) -> Vec<Vec<f64>> {
    // The number of states is an encoding of two pieces of
    // information: what square we're on, and how many consecutive
    // doubles we've rolled.
    let num_states = NUM_SQUARES * DOUBLES_JAIL;
    let mut matrix = Vec::new();
    for i_combined in 0..num_states {
	let (i, doubles) = decode(i_combined);
	let mut row = vec![0.0; num_states];
	for (j, &(p, pd)) in die_probability.iter().enumerate() {
	    let nxt = (i+j) % NUM_SQUARES;
	    // If the throw is not a double (which is what the
	    // probability p signifies), the doubles in the
	    // destination encoding is 0.
	    if p != 0.0 { fill_probabilities(&mut row, p, nxt, /* doubles = */ 0); }
	    if pd != 0.0 {
		// This means that we threw one less double than the
		// jail threshold in the source. One more double lands
		// us in jail.
		if doubles == (DOUBLES_JAIL - 1) { row[encode(JAIL, 0)] += pd; }
		// Otherwise, we encode one more double in the
		// destination.
		else { fill_probabilities(&mut row, pd, nxt, doubles+1); }
	    }
	}
	matrix.push(row);
    }
    matrix
}

// Returns a vector as long as all possible die totals. For each die
// total, we have two probabilities: the probability of getting that
// total without a double, and the probability of getting that total
// with a double.
fn build_die_probability(die_sides: u8) -> Vec<(f64, f64)> {
    let mut probabilities = vec![(0.0, 0.0); (2 * die_sides + 1) as usize];
    for a in 1..=die_sides {
	for b in 1..=die_sides {
	    let idx = (a+b) as usize;
	    if a == b { probabilities[idx].1 += 1.0; } else { probabilities[idx].0 += 1.0; }
	}
    }
    let total = (die_sides * die_sides) as f64;
    for i in 0..probabilities.len() {
	let (p1, p2) = probabilities[i];
	probabilities[i] = (p1 / total, p2 / total);
    }
    probabilities
}

// Fills out the correct spot in the probability row, according to the
// algorithm in the problem statement, plus some more. square is where
// we've landed on and doubles is the number of doubles we've had in a
// row (including the current roll). p is the probability of falling
// at the current square. The current square might not be the final
// spot because we could potentially go elsewhere if the current
// square is special.
fn fill_probabilities(probability_row: &mut Vec<f64>, p: f64, square: usize, doubles: usize) {
    // Special cases first.
    if CC.contains(&square) { // Community chest.
	probability_row[encode(GO, doubles)] += p / 16.0;
	probability_row[encode(JAIL, doubles)] += p / 16.0;
	probability_row[encode(square, doubles)] += p * 7.0 / 8.0;
    }
    else if CH.contains(&square) { // Chance.
	let idx = CH.iter().position(|&x| x == square).unwrap();
	probability_row[encode(GO, doubles)] += p / 16.0;
	probability_row[encode(JAIL, doubles)] += p / 16.0;
	probability_row[encode(24, doubles)] += p / 16.0;
	probability_row[encode(39, doubles)] += p / 16.0;
	probability_row[encode(5, doubles)] += p / 16.0;
	probability_row[encode(NEXT_R[idx], doubles)] += p / 8.0;
	probability_row[encode(NEXT_U[idx], doubles)] += p / 16.0;
	probability_row[encode(square, doubles)] += p * 3.0 / 8.0;
    } else if square == G2J { // Go to jail.
	probability_row[encode(JAIL, doubles)] += p;
    } else { // All other squares.
	probability_row[encode(square, doubles)] += p;
    }
}

fn encode(square: usize, doubles: usize) -> usize {
    doubles * NUM_SQUARES + square
}

fn decode(idx: usize) -> (usize, usize) {
    (idx % NUM_SQUARES, idx / NUM_SQUARES)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monopoly_most_popular() {
	assert_eq!(monopoly_most_popular(6).as_str(), "102400");
    }
}
