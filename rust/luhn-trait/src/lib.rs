// pub trait Luhn<T> where T: ToString {
pub trait Luhn<T> {
    fn valid_luhn(&self) -> bool;
}
impl<T> Luhn<T> for T where T: ToString {
    // the fn stself is just a copy from previous exo
    fn valid_luhn(&self) -> bool {
        let mut is_valid = true;
        let input = &self.to_string();
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
        is_valid
    }
}
// Here is the example of how to implement custom Luhn trait
// for the &str type. Naturally, you can implement this trait
// by hand for the every other type presented in the test suite,
// but your solution will fail if a new type is presented.
// Perhaps there exists a better solution for this problem?
// impl<'a> Luhn for &'a str {
//     fn valid_luhn(&self) -> bool {
//         unimplemented!("Determine if '{self}' is a valid credit card number.");
//     }
// }
//
