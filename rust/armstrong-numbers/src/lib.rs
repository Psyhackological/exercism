// kind-of-iterator solution
pub fn is_armstrong_number(num: u32) -> bool {
    let str_num: String = num.to_string();
    let num_length: usize = str_num.len();

    let mut armstrong_sum: u32 = 0;

    for digit in str_num.chars().map(|c| c.to_digit(10).unwrap()) {
        match digit.checked_pow(num_length as u32) {
            Some(val) => {
                armstrong_sum = match armstrong_sum.checked_add(val) {
                    Some(new_sum) => new_sum,
                    None => return false,
                }
            }
            None => return false,
        };
    }

    num == armstrong_sum
}
