#![warn(clippy::pedantic)]

#[must_use]
pub fn raindrops(n: u32) -> String {
    [(3, 'i'), (5, 'a'), (7, 'o')]
        .iter()
        .enumerate()
        .fold(String::new(), |mut acc, (i, t)| {
            if n % t.0 == 0 {
                acc.push_str(format!("Pl{}ng", t.1).as_str());
            } else if i == 2 && acc.is_empty() {
                acc.push_str(n.to_string().as_str());
            }
            acc
        })
}
