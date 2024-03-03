// u8 because 0..=9 is in 0..=255
fn luhn_digit_doubler(digit: u8) -> u8 {
    match digit {
        0 | 9 => digit, // 9 * 2 - / % 9 = 9 and 0 * anything = 0
        1 => 2,         // 9 > 2 = 2 * 1
        2 => 4,         // 9 > 4 = 2 * 2
        3 => 6,         // 9 > 6 = 2 * 3
        4 => 8,         // 9 > 8 = 2 * 4
        // Next everything doubled is > 9 so - / % 9 is needed
        5 => 1, // 1 = 2 * 5 - 9 or 1 = 2 * 5 % 9
        6 => 3, // 3 = 2 * 6 - 9 or 3 = 2 * 6 % 9
        7 => 5, // 5 = 2 * 7 - 9 or 5 = 2 * 7 % 9
        8 => 7, // 7 = 2 * 8 - 9 or 7 = 2 * 8 % 9
        _ => unreachable!("Congrats! You've done impossible and got single digit character > 9!"),
    }
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let cleaned_code = code // That's equal
        .chars() // Only characters please (any character, whitespace and invalid too)
        .filter(|c| !c.is_whitespace()) // No whitespace please
        .collect::<String>(); // Let's get this cleaned code

    // If any character is NOT ascii digit or cleaned_code <= 1 then
    if cleaned_code.chars().any(|c| !c.is_ascii_digit())
        || cleaned_code.len() <= 1
    // Preventing integer overflow
        || cleaned_code.len() as u32 > u32::MAX / 9
    {
        return false; // That's invalid
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
            if index % 2 == 0 {
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
