use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut s = HashSet::new();
    candidate.chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| s.insert(c.to_ascii_lowercase()))
}
