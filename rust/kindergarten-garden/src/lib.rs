// 2nd solution
// Thanks to:
// https://exercism.org/tracks/rust/exercises/kindergarten-garden/solutions/Tyraan
// https://exercism.org/tracks/rust/exercises/kindergarten-garden/solutions/sezoka

const CHILDREN: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let cup_start = CHILDREN.iter().position(|&s| s == student).unwrap_or(0) * 2;

    let char_map = |c: char| match c {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => "unknown",
    };

    diagram
        .lines()
        .flat_map(|line| line[cup_start..=cup_start + 1].chars().map(char_map))
        .collect()
}
