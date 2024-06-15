#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    string_digits
        .chars()
        .collect::<Vec<char>>()
        .windows(span)
        .fold(Ok(0u64), |acc, window| {
            let acc = acc?;
            let window_product = window
                .iter()
                .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(*c)))
                .fold(Ok(1u64), |acc, digit| Ok(u64::from(digit?) * acc?))?;

            if window_product > acc {
                Ok(window_product)
            } else {
                Ok(acc)
            }
        })
}
