// My first solution
pub fn series(digits: &str, len: usize) -> Vec<String> {
    if let true = len > 0 {
        digits
            .chars()
            .collect::<Vec<_>>()
            .windows(len)
            .map(|w| w.iter().collect())
            .collect()
    } else {
        vec!["".to_string(); digits.len() + 1]
    }
}
