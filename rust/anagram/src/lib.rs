use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut word_sorted: Vec<_> = word_lower.chars().collect();
    word_sorted.sort_unstable();

    possible_anagrams
        .iter()
        .cloned() // To convert &&str to &str
        .filter(|anagram| {
            let anagram_lower = anagram.to_lowercase();
            let mut anagram_sorted = anagram_lower.chars().collect::<Vec<_>>();
            anagram_sorted.sort_unstable();
            anagram_sorted == word_sorted && anagram_lower != word_lower
        })
        .collect()
}
