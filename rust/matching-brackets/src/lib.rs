#![warn(clippy::pedantic)]

fn closing(char: char) -> char {
    match char {
        '[' => ']',
        '{' => '}',
        '(' => ')',
        _ => unreachable!(),
    }
}

#[must_use]
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::<char>::new();

    for c in string.chars() {
        match c {
            '[' | '{' | '(' => stack.push(closing(c)),
            ']' | '}' | ')' if stack.pop() != Some(c) => return false,
            _ => (),
        }
    }

    stack.is_empty()
}
