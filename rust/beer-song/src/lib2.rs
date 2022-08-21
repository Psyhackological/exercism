pub fn verse(n: u32) -> String {
    // I wanted to match on (n - 1)
    // However that would mean u32 could be negative
    match n {
        2 => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n", n ),
        1 => format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"), 
        0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        _ => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, (n - 1)),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut beer_song = String::new();
    for n in (end..=start).rev() {
        beer_song.push_str(&verse(n));
        beer_song.push_str("\n");
    }

    // remove last \n character
    _ = beer_song.pop();
    beer_song
}
