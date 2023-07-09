pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    match list.len() {
        0 => (),
        // Not needed, but it's a small optimization.
        1 => return format!("And all for the want of a {0}.", list[0]),
        _ => {
            proverb = list
                .iter()
                .zip(list.iter().skip(1))
                .map(|(f, s)| format!("For want of a {f} the {s} was lost.\n"))
                .collect::<String>()
                + &format!("And all for the want of a {0}.", list[0]);
        }
    }

    proverb
}
