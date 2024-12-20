fn main() {
    let product = largest_product("\
	73167176531330624919225119674426574742355349194934\
	96983520312774506326239578318016984801869478851843\
	85861560789112949495459501737958331952853208805511\
	12540698747158523863050715693290963295227443043557\
	66896648950445244523161731856403098711121722383113\
	62229893423380308135336276614282806444486645238749\
	30358907296290491560440772390713810515859307960866\
	70172427121883998797908792274921901699720888093776\
	65727333001053367881220235421809751254540594752243\
	52584907711670556013604839586446706324415722155397\
	53697817977846174064955149290862569321978468622482\
	83972241375657056057490261407972968652414535100474\
	82166370484403199890008895243450658541227588666881\
	16427171479924442928230863465674813919123162824586\
	17866458359124566529476545682848912883142607690042\
	24219022671055626321111109370544217506941658960408\
	07198403850962455444362981230987879927244284909188\
	84580156166097919133875499200524063689912560717606\
	05886116467109405077541002256983155200055935729725\
	71636269561882670428252483600823257530420752963450\
	");
    println!("{}", product);
}

// We keep track of the "current product", which is the product of the
// last 13 digits before the current index, or the digits between the
// last '0' encountered and the current index, whichever is shorter
// (in terms of the number of digits in the product).
fn largest_product(digits: &str) -> u64 {
    let mut largest = 0u64;
    let mut current = 1u64;
    let mut idx: usize = 0;
    let mut considered = 0;
    while idx < digits.len() {
	let digit = digits[idx..(idx+1)].parse::<u64>().unwrap();
	if digit == 0 {
	    // In this case, we need to reset, because we will start a
	    // new group.
	    current = 1;
	    considered = 0;
	}
	else {
	    current *= digit as u64;
	    considered += 1;
	    // If the group under consideration has grown bigger than
	    // needed, we drop off the first member of the group.
	    if considered > 13 {
		current /= digits[(idx-13)..(idx-12)].parse::<u64>().unwrap();
		considered -= 1;
	    }
	    // The update condition. Since we're only interested in
	    // groups of 13 digits exactly.
	    if considered == 13 && current > largest {
		largest = current;
	    }
	}
	idx += 1;
    }
    return largest;
}
