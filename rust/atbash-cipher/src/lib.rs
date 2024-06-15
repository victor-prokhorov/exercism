// todo: change extract all `const` same for all ciphers exos
// 0                        26
// encoding
// abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz
// 0
// zyxwvutsrqponmlkjihgfedcba
//                          25
// + m
// 0 - 25
// 1 - 24
// 2 - 23
// +1 from beginning - -1 from the end of alphabet ('z' as u32 - 'a' as u32 - (index of letter in normal order))
// too bad there no char::from_u8 to write directly `b'a'` instead of 'a' as u32

pub fn encode(plain: &str) -> String {
    let m = 26 - 1; // b'z' - b'a' is 25, 25 indexes of difference between first and last
                    // not 26 as the len
    plain
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .enumerate()
        .flat_map(|(i, c)| {
            if let '1'..='9' = c {
                return vec![c];
            }
            let c = c.to_ascii_lowercase() as i32 - 'a' as i32;
            let x: u32 = (m - c).try_into().unwrap();
            if i % 5 == 0 && i != 0 {
                vec![' ', char::from_u32(x + 'a' as u32).unwrap()]
            } else {
                vec![char::from_u32(x + 'a' as u32).unwrap()]
            }
        })
        .collect::<String>()
}

pub fn decode(cipher: &str) -> String {
    // todo: split into separate fn the use with variable to check if we rm whitespace
    encode(cipher)
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect()
}
