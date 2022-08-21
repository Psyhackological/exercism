// "hacky solution"
pub fn is_armstrong_number(num: u32) -> bool {
    let mut armstrong_sum: u32 = 0;
    let str_num: String = num.to_string();
    let num_length: usize = str_num.len();

    for digit_char in str_num.chars() {
        // https://stackoverflow.com/a/41380607
        armstrong_sum += (digit_char as u32 - 48).pow(num_length as u32);
    }

    num == armstrong_sum
}
