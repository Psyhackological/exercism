// And then I get absolutely destroyed by GPT-4.
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|n| factors.iter().any(|f| *f != 0 && n % f == 0))
        .sum()
}
