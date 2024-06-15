pub fn number(user_number: &str) -> Option<String> {
    let mut digits: Vec<_> = user_number.chars().filter(|c| c.is_numeric()).collect();
    if digits.len() < 10 || digits.len() > 11 {
        return None;
    }
    if digits.len() == 11 {
        if digits.first().unwrap() == &'1' {
            digits.remove(0);
        } else {
            return None;
        }
    }
    debug_assert!(digits.len() == 10);
    // quit a messy flow, can probably refactor that
    match digits[0] {
        '2'..='9' => (),
        _ => return None,
    }
    match digits[3] {
        '2'..='9' => (),
        _ => return None,
    }
    let all_ok = digits.iter().all(|d| d.is_ascii_digit());
    if !all_ok {
        return None;
    }

    Some(digits.iter().collect())
}
