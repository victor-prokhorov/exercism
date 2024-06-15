use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut m = HashMap::new();
    // c.is_whitespace() || c ==',' || c ==':'
    words.split(|c: char| !c.is_alphanumeric() && c != '\'').collect::<Vec<_>>().iter().filter(|s| !s.is_empty()).for_each(|w| {
        dbg!(w);
        m.entry(w.trim_matches('\'').to_lowercase()).and_modify(|c| *c += 1).or_insert(1);
    });
    m
}
