// purely numerical approach
pub fn is_armstrong_number(num: u32) -> bool {
    // Avoids the overhead of converting the number to a string
    let mut n = num;
    // Computes the length of the number using logarithm, which is faster than converting to a string and finding length
    let len = ((num as f32).log10() as u32) + 1;
    let mut sum: u32 = 0;

    while n > 0 {
        // Performs direct arithmetic operation to get the digits, avoiding the need for iterating over string characters
        let digit = n % 10;
        // Uses `checked_pow()` to avoid potential overflow
        let powered = match digit.checked_pow(len) {
            Some(result) => result,
            None => return false, // Overflow occurred in power operation
        };
        // Uses `checked_add()` to avoid potential overflow
        sum = match sum.checked_add(powered) {
            Some(result) => result,
            None => return false, // Overflow occurred in addition operation
        };
        n /= 10;
    }

    num == sum
}
