use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // Closure that counts characters in a slice of lines (sequentially)
    let count_characters = |lines: &[&str]| {
        let mut char_frequencies = HashMap::new();
        for line in lines {
            for character in line
                .chars()
                .filter(|ch| ch.is_alphabetic()) // Only alphabetic
                // Lowercase conversion
                .map(|ch| ch.to_ascii_lowercase())
            {
                *char_frequencies.entry(character).or_default() += 1; // Add to or create new entry
            }
        }
        char_frequencies
    };

    if input.is_empty() || worker_count == 0 {
        return HashMap::new();
    }

    if input.len() <= 500 {
        return count_characters(input);
    }

    // For larger inputs, we use Rayon for parallelism
    input
        .par_chunks(input.len().max(worker_count))
        .map(count_characters)
        .reduce(
            HashMap::new, // Use the function directly instead of a closure
            |mut acc, elem| {
                for (character, count) in elem {
                    *acc.entry(character).or_default() += count;
                }
                acc
            },
        )
}
