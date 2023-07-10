// GPT-4 improvement
pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec!["".to_string(); digits.len() + 1],
        _ => digits
            .as_bytes() // convert &str to &[u8] without changing the underlying data
            .windows(len)
            .map(|w| std::str::from_utf8(w).unwrap().to_string()) // convert each window back to a string
            .collect(),
    }
}
