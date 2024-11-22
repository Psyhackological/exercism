/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
    const LENGTH: usize = ALPHABET.len();

    if sentence.is_empty() {
        return false;
    }

    let distinct_count = sentence
        .chars()
        .filter_map(|c| c.to_lowercase().next()) // Convert to lowercase
        .filter(|c| c.is_ascii_alphabetic()) // Only keep alphabetic characters
        .fold([false; LENGTH], |mut seen, c| {
            let index = (c as usize) - ('a' as usize);
            seen[index] = true;
            seen
        })
        .iter()
        .filter(|&&b| b) // Count the number of `true` entries
        .count();

    // 26 true values as in ALPHABET.len() (LENGTH)
    distinct_count == LENGTH
}
