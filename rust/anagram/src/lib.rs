use std::collections::{HashMap, HashSet};

pub trait CharCounter {
    fn get_chars_frequency(&self) -> HashMap<char, usize>;
}

pub trait AnagramsFinder<'a> {
    fn find_anagrams(&self, pattern_raw: &str) -> HashSet<&'a str>;
}

impl CharCounter for str {
    fn get_chars_frequency(&self) -> HashMap<char, usize> {
        self.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
    }
}

impl<'a> AnagramsFinder<'a> for [&'a str] {
    fn find_anagrams(&self, pattern_raw: &str) -> HashSet<&'a str> {
        let pattern = pattern_raw.to_lowercase();
        let pattern_map: HashMap<char, usize> = pattern.get_chars_frequency();
        self.iter()
            .copied()
            .filter(|word_raw| {
                let word = word_raw.to_lowercase();
                word != pattern && word.get_chars_frequency() == pattern_map
            })
            .collect()
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams.find_anagrams(word)
}
