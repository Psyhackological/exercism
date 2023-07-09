// "hacky solution"
pub fn is_armstrong_number(num: u32) -> bool {
    let mut armstrong_sum: Option<u32> = Some(0);
    let str_num: String = num.to_string();
    let num_length: usize = str_num.len();

    for digit_char in str_num.chars() {
        let digit = digit_char as u32 - 48;
        // https://stackoverflow.com/a/41380607
        let powered = match digit.checked_pow(num_length as u32) {
            Some(result) => result,
            None => return false, // Overflow occurred in power operation
        };
        armstrong_sum = match armstrong_sum.unwrap().checked_add(powered) {
            Some(result) => Some(result),
            None => return false, // Overflow occurred in addition operation
        };
    }

    match armstrong_sum {
        Some(sum) => num == sum,
        None => false, // Overflow occurred
    }
}
