/// The function handles spaces, hyphens, and certain punctuations as word separators, and accounts for camel case words.
/// It iterates through the character stream of the phrase exactly once, making it efficient.
/// Also, further optimized abbreviation function with pre-allocated string size.
pub fn abbreviate(phrase: &str) -> String {
    // Estimate the number of words for initial string allocation.
    // Simplify initial capacity estimation to a value likely to cover the acronym size without complex calculations.
    // https://doc.rust-lang.org/std/string/struct.String.html#method.with_capacity
    let estimated_size = phrase.len() / 2;
    let mut acronym = String::with_capacity(estimated_size);
    let mut new_word = true;
    let mut previous_char = ' ';

    for c in phrase.chars() {
        if c.is_whitespace() || c == '-' || c == '_' {
            new_word = true;
        } else if new_word
            || (c.is_uppercase() && !previous_char.is_uppercase() && !previous_char.is_whitespace())
        {
            acronym.push(c.to_ascii_uppercase());
            new_word = false;
        }
        previous_char = c;
    }

    acronym
}

// Stupid HTML Camel Case thingy ruined it
//
// pub fn abbreviate(phrase: &str) -> String {
//     let cleaned_phrase = phrase
//         .chars()
//         .map(|c| if c == '-' { ' ' } else { c })
//         .filter(|&c| c.is_alphabetic() || c == ' ')
//         .collect::<String>();
//
//     cleaned_phrase
//         .split_whitespace()
//         .filter_map(|word| word.chars().next())
//         .map(|c| c.to_ascii_uppercase())
//         .collect()
// }
