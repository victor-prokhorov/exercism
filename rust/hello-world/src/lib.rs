#![warn(clippy::pedantic)]

#[must_use]
pub fn hello() -> &'static str {
    "Hello, World! fail just to test"
}
