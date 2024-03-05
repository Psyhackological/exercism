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

    if let Some(&invalid_digit) = number.iter().find(|&&digit| digit >= from_base) {
        return Err(Error::InvalidDigit(invalid_digit));
    }

    let mut decimal_value = number
        .iter()
        .rev()
        .enumerate()
        // acc = accumulator, idx = index, &val = value
        .fold(0, |acc, (idx, &val)| acc + val * from_base.pow(idx as u32));

    let mut converted_digits = Vec::new();
    // Iteration 1:
    // result.push(42 % 10) pushes 2 to result.
    // decimal becomes 42 / 10, which is 4.
    // Iteration 2:
    // result.push(4 % 10) pushes 4 to result.
    // decimal becomes 4 / 10, which is 0.
    while decimal_value > 0 {
        converted_digits.push(decimal_value % to_base); // Push the remainder of decimal divided by to_base into result
        decimal_value /= to_base; // Divide decimal by to_base, updating decimal for the next iteration
    }

    // No digits were added, which implies the number is zero
    if converted_digits.is_empty() {
        converted_digits.push(0);
    } else {
        // We push from "lowest" to "highest" so the final digit is reversed
        converted_digits.reverse();
    }

    Ok(converted_digits)
}
