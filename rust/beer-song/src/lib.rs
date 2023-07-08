pub fn verse(n: u32) -> String {
    // I wanted to match on (n - 1)
    // However that would mean u32 could be negative
    match n {
        2 => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n", n ),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(), 
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        _ => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, (n - 1)),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}
