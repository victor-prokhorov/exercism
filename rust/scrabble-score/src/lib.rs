use std::collections::HashMap;

#[must_use]
pub fn score(word: &str) -> u64 {
    let mut result: u64 = 0;
    let map: HashMap<Vec<char>, u64> = HashMap::from([
        (vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'], 1),
        (vec!['D', 'G'], 2),
        (vec!['B', 'C', 'M', 'P'], 3),
        (vec!['F', 'H', 'V', 'W', 'Y'], 4),
        (vec!['K'], 5),
        (vec!['J', 'X'], 8),
        (vec!['Q', 'Z'], 10),
    ]);
    for c in word.chars() {
        if c.is_alphabetic() {
            for (k, v) in &map {
                if k.contains(&c.to_ascii_uppercase()) {
                    result += v;
                }
            }
        }
    }
    result
}
