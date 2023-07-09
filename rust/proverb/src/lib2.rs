pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    match list.len() {
        0 => (),
        1 => return format!("And all for the want of a {0}.", list[0]),
        _ => {
            proverb = list
                .iter()
                .zip(list.iter().skip(1))
                .map(|(f, s)| format!("For want of a {f} the {s} was lost."))
                .collect::<Vec<String>>()
                .join("\n");
            proverb.push_str(&format!("\nAnd all for the want of a {0}.", list[0]));
        }
    }

    proverb
}
