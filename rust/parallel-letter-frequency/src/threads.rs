// Inspiration to understand this solution with better naming
// https://exercism.org/tracks/rust/exercises/parallel-letter-frequency/solutions/rsalmei
use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // a closure
    let count_characters = |lines: &[&str]| {
        let mut char_frequencies = HashMap::new();
        for line in lines {
            for character in line
                .chars()
                .filter(|ch| ch.is_alphabetic()) // only alphabetic
                .map(|ch| ch.to_ascii_lowercase())
            // only lowercase
            {
                *char_frequencies.entry(character).or_default() += 1; // add to or create new entry
            }
        }
        char_frequencies
    };

    match input.len() {
        0 => HashMap::new(),                                       // empty input
        small_size if small_size < 500 => count_characters(input), // use one-threaded closure
        // threads lets go
        _ => thread::scope(|scope| {
            let mut worker_handles = Vec::with_capacity(worker_count);
            // +1 ensures even distribution of leftover lines when dividing texts.
            for chunked_lines in input.chunks(input.len() / worker_count + 1) {
                worker_handles.push(scope.spawn(|| count_characters(chunked_lines)))
            }

            // instead of empty HashMap we get 1 worker's value
            let mut final_frequencies = worker_handles
                .pop()
                .expect("No worker handles available")
                .join()
                .expect("Thread panicked while joining");

            for handle in worker_handles {
                handle
                    .join()
                    .expect("Thread panicked while joining")
                    .into_iter()
                    .for_each(|(character, count)| {
                        // or_default returns usize which is 0 and then adds the count
                        *final_frequencies.entry(character).or_default() += count;
                    })
            }

            final_frequencies
        }),
    }
}
