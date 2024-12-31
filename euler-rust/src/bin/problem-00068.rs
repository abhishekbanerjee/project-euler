fn main() {
    // Solve by hand.
    //
    // We want a 16-digit string. This means 10 will be on the
    // external ring. Also, we want the maximum string, so we will put
    // the higher numbers in the external ring and the first external
    // number would be 6 (since the lowest external number comes first
    // in the representation).
    //
    // The total sum is 10 + 9 + 8 + 7 + 6 + (5 + 4 + 3 + 2 + 1) * 2 =
    // 70. So the sum along each arm is 14. To maximize the
    // representation, let's put 5 next to 6. So Arm 1 becomes: [6, 5,
    // 3]. The rest of the arms fall in place because there is only
    // one solution that uses the numbers only once and keeps the sums
    // along the arms at 14.
    //
    // The solution, visualized:
    //     6
    //      \
    //       5  10
    //      / \ /
    //     2   3
    //   / |   |
    //  7  4 - 1 - 9
    //     |
    //     8
    println!("6531031914842725");
}
