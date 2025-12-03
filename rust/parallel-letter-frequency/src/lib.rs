use std::any::Any;
use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::<char, usize>::new();
    }

    let worker_count = worker_count.max(1);

    thread::scope(|s| {
        let chunk_size = input.len().div_ceil(worker_count);
        let mut handles: Vec<_> = input
            .chunks(chunk_size)
            .map(|chunk| {
                s.spawn(move || {
                    let mut proxy_map = HashMap::new();
                    chunk
                        .iter()
                        .flat_map(|line| line.chars())
                        .filter(|char| char.is_alphabetic())
                        .flat_map(|char_slice| char_slice.to_lowercase())
                        .for_each(|char| {
                            *proxy_map.entry(char).or_insert(0) += 1;
                        });
                    proxy_map
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|x| x.join().unwrap())
            .fold(HashMap::new(), |mut acc, map| {
                for (char, count) in map {
                    *acc.entry(char).or_insert(0) += count;
                }
                acc
            })
    })
}
