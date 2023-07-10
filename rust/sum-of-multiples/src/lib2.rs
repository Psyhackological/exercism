use std::collections::HashSet;

// First solution
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let sequences: Vec<_> = factors
        .iter()
        .filter(|f| **f > 0_u32)
        .map(|&f| (f..limit).step_by(f as usize))
        .collect();

    let sum: u32 = sequences
        .into_iter()
        .flatten()
        .collect::<HashSet<_>>()
        .iter()
        .sum();

    sum
}
