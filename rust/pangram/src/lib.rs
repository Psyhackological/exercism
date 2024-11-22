const ASCII_A_LOWER: u8 = 97; // 'a' ASCII representation.
const ASCII_A_UPPER: u8 = 65; // 'A' ASCII representation.
const ALL_26_BITS_SET: u32 = 0b00000011_11111111_11111111_11111111;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut letter_flags = 0b00000000_00000000_00000000_00000000;

    for letter in sentence.chars() {
        if letter.is_ascii_lowercase() {
            letter_flags |= letter_to_bit(letter as u8, ASCII_A_LOWER);
        } else if letter.is_ascii_uppercase() {
            letter_flags |= letter_to_bit(letter as u8, ASCII_A_UPPER);
        }
    }
    letter_flags == ALL_26_BITS_SET
}

/// Convert an ASCII letter to its corresponding bit in the bitfield.
fn letter_to_bit(letter: u8, base: u8) -> u32 {
    1 << (letter - base)
}
