#![warn(clippy::pedantic)]

#[must_use]
/// assuming `phrase` is always ascii
/// optimized for speed
pub fn abbreviate(phrase: &str) -> String {
    if phrase.is_empty() {
        return String::new();
    }

    let mut string: String = String::new();

    let bytes = phrase.as_bytes();

    string.push(
        phrase
            .chars()
            .next()
            .expect("We return empty `String` if `phrase` is empty."),
    );

    for i in 0..phrase.len() - 1 {
        let l = bytes[i] as char;
        let r = bytes[i + 1] as char;

        if (l.is_ascii_whitespace() || l == '-' || l == '_') && r.is_ascii_alphabetic()
            || l.is_ascii_lowercase() && r.is_ascii_uppercase()
        {
            string.push(r.to_ascii_uppercase());
        }
    }

    string
}
