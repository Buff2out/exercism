pub fn abbreviate(phrase: &str) -> String {
    std::iter::once(' ')
        .chain(phrase.chars())
        .zip(phrase.chars())
        .filter_map(|(prev, curr)| {
            let is_word_started = curr.is_alphabetic()
                && ((!prev.is_alphabetic() && prev != '\'')
                    || (prev.is_lowercase() && curr.is_uppercase()));
            if is_word_started {
                Some(curr.to_ascii_uppercase())
            } else {
                None
            }
        })
        .collect()
}
