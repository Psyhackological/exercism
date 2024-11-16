// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_hashmap = HashMap::new();
    let mut note_hashmap = HashMap::new();

    // Thanks Insomnia#7030
    for m in magazine {
        for word in m.split_whitespace() {
            magazine_hashmap
                .entry(word)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    for word in note.iter().flat_map(|s| s.split_whitespace()) {
        note_hashmap
            .entry(word)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    // Thanks skeletizzle#8654
    note_hashmap
        .iter()
        .all(|(word, count)| Some(count) <= magazine_hashmap.get(word))
}
