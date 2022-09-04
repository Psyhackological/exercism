// Thanks https://exercism.org/profiles/Jercik
fn is_prime(number: u32) -> bool {
    // We don't need to check higher than max_divisor = √number
    // Ah those smart mathematicians, not like me
    let max_divisor = (number as f32).sqrt() as u32;

    // 1. Create range 2 to max divisor (including it)
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
    // 2. Using `any` method that is satisfied if at least one of the elements passes the condition
    // 3. We invert it - because prime numbers CANNOT BE DIVIDED BY SOMETHING OTHER THAN THEMSELVES
    // (or 1, but this is always true for u32)
    // 4. In summary:
    //  - check this range
    //  - if there are elements that can divide this number
    //  - if there were none -> false
    //  - if false then !false -> true
    !(2..=max_divisor).any(|d| number % d == 0)
}

pub fn nth(n: u32) -> u32 {
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth
    // 1. Create range from 2 to u32::MAX (x named in the future methods)
    // 2. Filter values that satisfy code above
    // (dereference, because closure takes a reference and the compiler helped me)
    // 3. Take from this iterator nth element
    // 4. It gives back Option<Self::Item>, so we need to unwrap it
    let found: u32 = (2..=u32::MAX)
        .filter(|x| is_prime(*x))
        .nth(n as usize)
        .unwrap();

    found
}
