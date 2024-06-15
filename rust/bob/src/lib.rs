#![warn(clippy::pedantic)]

#[must_use]
pub fn reply(message: &str) -> &str {
    match (
        message.trim().is_empty(),
        message.to_uppercase() == message && message.contains(char::is_alphabetic),
        message.trim().ends_with('?'),
    ) {
        (true, _, _) => "Fine. Be that way!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, true, _) => "Whoa, chill out!",
        (_, _, true) => "Sure.",
        _ => "Whatever.",
    }
}
