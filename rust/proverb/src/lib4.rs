// Not my solution - https://exercism.org/tracks/rust/exercises/proverb/solutions/nickers
// I send this solution to my GitHub repo and to archive it.
use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(word) => list
            .windows(2)
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            .chain(once(format!("And all for the want of a {}.", word)))
            .collect(),
    }
}
