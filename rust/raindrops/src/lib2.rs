pub fn raindrops(n: u32) -> String {
    let mut sound = String::new();

    let divisible_by_3 = n % 3 == 0;
    let divisible_by_5 = n % 5 == 0;
    let divisible_by_7 = n % 7 == 0;

    if divisible_by_3 {
        sound += "Pling";
    }

    if divisible_by_5 {
        sound += "Plang";
    }

    if divisible_by_7 {
        sound += "Plong";
    }

    if sound.is_empty() {
        return n.to_string();
    }

    sound
}
