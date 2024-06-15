pub fn rotate(input: &str, key: i8) -> String {
    // key is 0..=25 the length of the ascii alphabet
    // ROT + <key>
    // example: ROT13
    // input.chars().map(|c| c.to_digit(10).unwrap() as i8).map(|d| d - key).map(|d| char::from(d as u8)).collect()
    // only on alphabetic chars
    input.chars().map(|c| {
        dbg!(&c);
        // dbg!(b'a', b'A', b'z', b'Z');
        //[src\lib.rs:9] b'a' = 97
        //[src\lib.rs:9] b'A' = 65
        //[src\lib.rs:9] b'z' = 122
        //[src\lib.rs:9] b'Z' = 90
        if !c.is_alphabetic() { return c; }
        // if alphabetic shift
        // handle both cases
        // dbg!(char::from((c as i32 + key as i32) as u8));
        let d = match c {
            'a'..='z' => (c as i32 + (key as i32%26) - 97) % 26 + 97,
            'A'..='Z' => (c as i32 + (key as i32%26) - 65) % 26 + 65,
            _=>panic!("nope"),
        };
        dbg!(char::from(d as u8))
    }).collect()

}
