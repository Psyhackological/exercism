use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut char_count = HashMap::new();

    input
        .iter() // we want iterate over &[&str]
        .flat_map(|l| l.split_whitespace()) // we flatten it, create iterator by whitespace
        .for_each(|w| {
            // now we have a word
            w.to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .for_each(|ch| {
                    *char_count.entry(ch).or_insert(0) += 1;
                });
        });

    char_count
}
