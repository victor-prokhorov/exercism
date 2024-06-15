#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let input: Vec<&str> = input.split_terminator('\n').collect();
    let height = input.len();
    if height % 4 != 0 {
        return Err(Error::InvalidRowCount(input.len()));
    }
    for row in &input {
        if row.len() % 3 != 0 {
            return Err(Error::InvalidColumnCount(input.len()));
        }
    }
    let width = input.get(0).unwrap().len();

    let mut digits: Vec<String> = Vec::new();
    let mut digit = String::new();

    for i in (0..height + 4).skip(4).step_by(4) {
        for j in (0..width).step_by(3) {
            for (k, row) in input.iter().enumerate().take(i).skip(i - 4) {
                if k % 4 != 0 {
                    digit.push('\n');
                }
                digit.push_str(row.chars().skip(j).take(3).collect::<String>().as_str());
            }
            digits.push(
                match digit.as_str() {
                    " _ \n| |\n|_|\n   " => "0",
                    "   \n  |\n  |\n   " => "1",
                    " _ \n _|\n|_ \n   " => "2",
                    " _ \n _|\n _|\n   " => "3",
                    "   \n|_|\n  |\n   " => "4",
                    " _ \n|_ \n _|\n   " => "5",
                    " _ \n|_ \n|_|\n   " => "6",
                    " _ \n  |\n  |\n   " => "7",
                    " _ \n|_|\n|_|\n   " => "8",
                    " _ \n|_|\n _|\n   " => "9",
                    _ => "?",
                }
                .to_owned(),
            );
            digit.clear();
        }
        if height > 4 && i % 4 == 0 && i != height {
            digits.push(",".to_string());
        }
    }

    Ok(digits.concat())
}
