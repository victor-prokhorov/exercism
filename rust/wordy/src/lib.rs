pub fn answer(command: &str) -> Option<i32> {
    let mut v: Vec<String> = command
        .split(' ')
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.to_string())
        .collect();
    // probabaly `VecDeque` would be better
    v.remove(0); // 'What'
    v.remove(0); // 'is'
    if let Some(last) = v.last_mut() {
        let (without_question_mark, _) = last.split_at(last.len() - 1);
        *last = without_question_mark.to_string();
    } else {
        return None;
    }
    // if v.len() == 1 {
    //     return Some(v.first().unwrap().parse::<i32>().unwrap());
    // }
    dbg!(&v);
    // if v.len() < 3 {
    //     return None;
    // }
    while v.len() >= 3 {
        match v.get(1).unwrap().as_str() {
            "plus" => {
                let r = v[0].parse::<i32>().ok()? + v[2].parse::<i32>().ok()?;
                for _ in 0..3 {
                    v.remove(0);
                }
                v.insert(0, r.to_string());
            }
            "minus" => {
                let r = v[0].parse::<i32>().ok()? - v[2].parse::<i32>().ok()?;
                for _ in 0..3 {
                    v.remove(0);
                }
                v.insert(0, r.to_string());
            }
            "multiplied" => {
                let r = v[0].parse::<i32>().ok()? * v[3].parse::<i32>().ok()?;
                for _ in 0..4 {
                    v.remove(0);
                }
                v.insert(0, r.to_string());
            }
            "divided" => {
                let r = v[0].parse::<i32>().ok()? / v[3].parse::<i32>().ok()?;
                for _ in 0..4 {
                    v.remove(0);
                }
                v.insert(0, r.to_string());
            }
            _ => return None,
        }
    }
    if v.len() == 1 {
        return Some(v.first().unwrap().parse::<i32>().unwrap());
    }
    None
}
