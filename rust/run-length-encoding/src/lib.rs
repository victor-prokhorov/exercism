pub fn encode(source: &str) -> String {
    let mut r = "".to_string();
    let mut char_repeat_count = 0;
    let mut prev_char: char = '*';
    for (c, i) in source.chars().enumerate() {
        if prev_char == i {
            char_repeat_count += 1;
        } else {
            if prev_char != '*' {
                if char_repeat_count > 1 {
                    r.push_str(&char_repeat_count.to_string());
                }
                r.push(prev_char);
            }
            char_repeat_count = 1;
        }

        if c == source.len() - 1 {
            if char_repeat_count > 1 {
                r.push_str(&char_repeat_count.to_string());
            }
            r.push(i);
        }
        prev_char = i;
    }
    r
}

pub fn decode(source: &str) -> String {
    let mut r = "".to_string();
    let mut prev_is_num = false;
    let mut string_number = "".to_string();
    for i in source.chars() {
        match i {
            '0'..='9' => {
                // dbg!(prev_is_num);
                string_number.push(i);
                prev_is_num = true;
            }
            'a'..='z' | 'A'..='Z' | ' ' => {
                // dbg!(prev_is_num);
                if prev_is_num {
                    // todo: handle unwrap instead of bool
                    let n: u32 = string_number.parse().unwrap();
                    // dbg!(n);
                    for _ in 0..n {
                        r.push(i);
                    }
                    string_number.clear();
                } else {
                    r.push(i);
                }

                prev_is_num = false;
            }
            _ => panic!(),
        }
    }
    r
}
