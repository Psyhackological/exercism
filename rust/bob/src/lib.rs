pub fn reply(message: &str) -> &str {
    let message = message.trim();

    let reply_empty: &str = "Fine. Be that way!";
    let reply_upper_and_question: &str = "Calm down, I know what I'm doing!";
    let reply_upper: &str = "Whoa, chill out!";
    let reply_question: &str = "Sure.";
    let reply_default: &str = "Whatever.";

    let is_question: bool = message.ends_with('?');
    // To yell you need to have letters and their must be uppercased
    // 1st condition - is_uppercased?
    // 2nd condition - does it have a char that is a letter? (not emoji or digit)
    let is_uppercase: bool =
        message.to_uppercase() == message && message.chars().any(char::is_alphabetic);

    match (message, is_question, is_uppercase) {
        ("", _, _) => reply_empty,
        (_, true, true) => reply_upper_and_question,
        (_, true, _) => reply_question,
        (_, _, true) => reply_upper,
        _ => reply_default,
    }
}
