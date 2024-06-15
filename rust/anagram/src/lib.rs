use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut s = HashSet::new();
    for candidate in possible_anagrams {
        if is_anagram(&word.to_lowercase(), &candidate.to_lowercase()) {
            s.insert(*candidate);
        }
    }
    s
}

fn is_anagram(word: &str, candidate: &str) -> bool {
    if word.len() != candidate.len() || word == candidate {
        return false;
    }
    let mut m = HashMap::new();
    for c in candidate.chars() {
        *m.entry(c).or_insert(0) += 1;
    }
    for c in word.chars() {
        match m.get_mut(&c) {
            Some(x) if *x > 0 => *x -= 1,
            _ => return false,
        }
    }
    true
}
