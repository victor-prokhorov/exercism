#![warn(clippy::pedantic)]

#[must_use]
/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    println!("code: {code}");

    let mut buffer: Vec<u8> = vec![];

    for c in code.chars() {
        if c == ' ' {
            continue;
        }

        if let Some(digit) = c.to_digit(10) {
            if (0..=9).contains(&digit) {
                #[allow(clippy::cast_possible_truncation)]
                buffer.push(digit as u8);
            }
        } else {
            return false;
        }
    }

    if buffer.len() < 2 {
        return false;
    }

    println!("{buffer:?}");

    buffer
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &number)| {
            if i % 2 == 0 {
                return number;
            }
            let double = 2 * number;
            if double > 9 {
                return double - 9;
            }
            double
        })
        .sum::<u8>()
        % 10
        == 0
}
