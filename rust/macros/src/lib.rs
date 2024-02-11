use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap {
    () => HashMap::new();
    ($a: expr, $b: expr) => HashMap::new();
    () => {
        let mut hm = HashMap::new();
    };
}

// Dowolny klucz
// Dowolna wartosc
// Strzalka oddzielajaca
// Druugie to expression
// hashmap!('a' => 3, 'b' => 11, 'z' => 32)
// {
//    let mut hm = HashMap::new();
//    hm.insert('a', 3);
//    hm.insert('b', 11);
//    hm.insert('z', 32);
//    hm
// }
