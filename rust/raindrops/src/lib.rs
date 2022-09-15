pub fn raindrops(n: u32) -> String {
    let mut sound = String::new();

    let has_factor = |factor| n % factor == 0;

    if has_factor(3) {
        sound.push_str("Pling");
    }

    if has_factor(5) {
        sound.push_str("Plang");
    }

    if has_factor(7) {
        sound.push_str("Plong");
    }

    if sound.is_empty() {
        return n.to_string();
    }

    sound
}
