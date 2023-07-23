// 1st solution
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let digram_lines = diagram.split('\n').collect::<Vec<&str>>();

    let start: usize = match student {
        "Alice" => 0,
        "Bob" => 2,
        "Charlie" => 4,
        "David" => 6,
        "Eve" => 8,
        "Fred" => 10,
        "Ginny" => 12,
        "Harriet" => 14,
        "Ileana" => 16,
        "Joseph" => 18,
        "Kincaid" => 20,
        "Larry" => 22,
        _ => panic!("Unknown student"),
    };

    let mut cups: Vec<&'static str> = Vec::new();

    for line in digram_lines {
        let two_plants: Vec<&str> = line
            .chars()
            .skip(start)
            .take(2)
            .map(|c| match c {
                'V' => "violets",
                'R' => "radishes",
                'C' => "clover",
                'G' => "grass",
                _ => "unknown",
            })
            .collect();

        cups.extend(two_plants);
    }

    cups
}
