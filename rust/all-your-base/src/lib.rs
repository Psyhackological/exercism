#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // No base like 0 or 1
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }

    // No base like 0 or 1
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    // Check for the edge case where the input is empty or all zeros
    if number.is_empty() || number.iter().all(|&digit| digit == 0) {
        return Ok(vec![0]);
    }

    let mut decimal = 0;
    for &digit in number {
        // For example, in base 2, valid digits are 0 and 1.
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
        // For example
        // Start with decimal = 0.
        // First digit:  1 -> decimal = 0 * 2 + 1 = 1.
        // Second digit: 0 -> decimal = 1 * 2 + 0 = 2.
        // Third digit:  1 -> decimal = 2 * 2 + 1 = 5.
        // Fourth digit: 0 -> decimal = 5 * 2 + 0 = 10.
        // Fifth digit:  1 -> decimal = 10 * 2 + 1 = 21.
        // Sixth digit:  0 -> decimal = 21 * 2 + 0 = 42.
        // Now, decimal = 42.
        decimal = decimal * from_base + digit;
    }

    let mut result = Vec::new();
    // Iteration 1:
    // result.push(42 % 10) pushes 2 to result.
    // decimal becomes 42 / 10, which is 4.
    // Iteration 2:
    // result.push(4 % 10) pushes 4 to result.
    // decimal becomes 4 / 10, which is 0.
    while decimal > 0 {
        result.push(decimal % to_base); // Push the remainder of decimal divided by to_base into result
        decimal /= to_base; // Divide decimal by to_base, updating decimal for the next iteration
    }

    // That means ran once
    if result.is_empty() {
        result.push(0);
    } else {
        // We push from "lowest" to "highest" so the final digit is reversed
        result.reverse();
    }

    Ok(result)
}
