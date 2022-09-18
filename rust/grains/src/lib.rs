pub fn square(s: u32) -> u64 {
    match s {
        0 | 65.. => panic!("Square must be between 1 and 64"),
        _ => u64::pow(2, s - 1),
    }
}

pub fn total() -> u64 {
    // 0. Solution
    // The largest value that can be represented by this integer type (2^64 − 1) == u64::MAX
    u64::MAX

    // 1. Solution
    // (u64::pow(2, 63) - 1) * 2 + 1

    // 2. Solution
    // (u128::pow(2, 64) - 1) as u64

    // 3. Solution
    // match 2_u64.checked_pow(64) {
    //     Some(value) => value - 1,
    //     None => u64::MAX,
    // }
    //
    // 4. Solution
    // (1..=64).map(square).sum()
}
