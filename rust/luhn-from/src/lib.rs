pub struct Luhn<T> {
    buffer: Vec<u8>,
    is_valid: bool,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Luhn<T>
where
    T: ToString,
{
    pub fn from(input: T) -> Self {
        let mut is_valid = true;
        let input = input.to_string();
        let buffer: Vec<u8> = input
            .chars()
            // todo: probably filter_map in one go
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_digit(10))
            // todo: there certainly a better method for that
            .take_while(|&c| if c.is_some() { true } else { is_valid = false;false }).map(|x| x.unwrap() as u8).collect();
            // .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            // .collect();
        dbg!(&buffer);
        if buffer == vec![0] || buffer.len() == 1 { is_valid = false;}

        let checksum = buffer
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
            .sum::<u8>();

        if checksum % 10 == 0 && is_valid {}else{is_valid=false}

        Self {
            buffer,
            is_valid,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn is_valid(&self) -> bool {
        return self.is_valid;
        // dbg!(&self.buffer);

    }
}

// copy from luhn exo
//pub fn is_valid(code: &str) -> bool {
//    println!("code: {code}");
//
//    let mut buffer: Vec<u8> = vec![];
//
//    for c in code.chars() {
//        if c == ' ' {
//            continue;
//        }
//
//        if let Some(digit) = c.to_digit(10) {
//            if (0..=9).contains(&digit) {
//                #[allow(clippy::cast_possible_truncation)]
//                buffer.push(digit as u8);
//            }
//        } else {
//            return false;
//        }
//    }
//
//    if buffer.len() < 2 {
//        return false;
//    }
//
//    println!("{buffer:?}");
//
//    buffer
//        .iter()
//        .rev()
//        .enumerate()
//        .map(|(i, &number)| {
//            if i % 2 == 0 {
//                return number;
//            }
//            let double = 2 * number;
//            if double > 9 {
//                return double - 9;
//            }
//            double
//        })
//        .sum::<u8>()
//        % 10
//        == 0
//}
//
