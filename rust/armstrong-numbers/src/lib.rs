// "hacky solution"
pub fn is_armstrong_number(num: u32) -> bool {
    let str_num: String = num.to_string();
    let num_length: usize = str_num.len();

    let armstrong_sum = str_num
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d.pow(num_length as u32))
        .sum();

    num == armstrong_sum
}
