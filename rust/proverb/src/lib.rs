#![warn(clippy::pedantic)]

#[must_use]
pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(str) => list
            .windows(2)
            .map(|s| format!("For want of a {} the {} was lost.\n", s[0], s[1]))
            .chain(std::iter::once(format!(
                "And all for the want of a {}.",
                str
            )))
            .collect(),
    }
}
