#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let is_sublist =
        |a: &[T], b: &[T]| a.is_empty() || b.windows(a.len()).any(|window| window == a);

    use Comparison::*;
    match (first_list, second_list) {
        (a, b) if a == b => Equal,
        (a, b) if is_sublist(a, b) => Sublist,
        (a, b) if is_sublist(b, a) => Superlist,
        _ => Unequal,
    }
}
