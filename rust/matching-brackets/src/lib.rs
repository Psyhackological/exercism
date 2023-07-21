pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets_vec: Vec<char> = Vec::with_capacity(string.len());

    for character in string.chars() {
        match character {
            '(' | '[' | '{' => brackets_vec.push(character),
            ')' => {
                if brackets_vec.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if brackets_vec.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if brackets_vec.pop() != Some('{') {
                    return false;
                }
            }
            _ => continue,
        }
    }
    brackets_vec.is_empty()
}
