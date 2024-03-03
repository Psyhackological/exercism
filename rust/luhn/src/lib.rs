// u8 because 0..=9 is in 0..=255
fn luhn_digit_doubler(digit: u8) -> u8 {
    match digit {
        0 | 9 => digit, // 9 * 2 - / % 9 = 9 and 0 * anything = 0
        1..=4 => digit * 2,
        // Next everything doubled is > 9 so - / % 9 is needed
        5..=9 => digit * 2 % 9,
        _ => {
            unreachable!("Congrats! You've done the impossible and got single digit character > 9!")
        }
    }
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let cleaned_code = code // That's equal
        .chars() // Only characters please (any character, whitespace and invalid too)
        .filter(|c| !c.is_whitespace()) // No whitespace please
        .collect::<String>(); // Let's get this cleaned code

    // If any character is NOT ascii digit or cleaned_code <= 1 or
    if cleaned_code.chars().any(|c| !c.is_ascii_digit())
        || cleaned_code.len() <= 1
    // integer overflow while summing then
        || cleaned_code.len() as u32 > u32::MAX / 9
    {
        return false; // that's invalid
    }

    cleaned_code
        .chars() // Getting characters
        .rev() // Starting from right (right to left)
        .enumerate() // To get even and odd characters position
        // tuple: (usize, char)
        .map(|(index, ch)| {
            // Safe due to prior validation but better safe than sorry
            let digit = ch.to_digit(10).unwrap_or(0) as u8;

            // even
            if index & 1 == 0 { // Using bitwise AND to check if index is even
                digit as u32
            // odd
            } else {
                luhn_digit_doubler(digit) as u32
            }
        })
        .sum::<u32>() // u32 because u8 can easily overflow 
                      // 99999999999999999999999999999 -> 261 > 255
        % 10
        == 0
}
