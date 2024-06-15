pub fn is_pangram(sentence: &str) -> bool {
    let mut counts = 0;

    for ch in sentence.chars() {
        if ch.is_ascii_alphabetic() {
            let ch = ch.to_ascii_lowercase();
            let n = 1 << ch as u8 - b'a';
            counts |= n;
        }
    }

    // 32 - 26 = we have to unset 6 first bits
    counts == 0b_0000_0011_1111_1111_1111_1111_1111_1111
}
