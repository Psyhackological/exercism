pub fn is_leap_year(year: u64) -> bool {
    // divisible by 4
    // and not divisible by 100
    // or if so divisible by 400
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}
