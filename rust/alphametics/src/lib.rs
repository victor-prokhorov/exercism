use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut values = parse(input);
    values.sort();
    let mut ans = vec![0; values.len()];
    find(0, &mut ans, &(0..=9).collect::<Vec<u8>>(), &values)
}

fn parse(input: &str) -> Vec<(char, i64, bool)> {
    let mut ans = HashMap::new();
    let mut leading = HashSet::new();
    let mut prev = ' ';
    let mut value = -1;

    for c in input.chars().rev() {
        match c {
            'A'..='Z' => {
                ans.entry(c).and_modify(|i| *i += value).or_insert(value);
                value *= 10;
                prev = c;
            }
            _ => {
                value = 1;
                leading.insert(prev);
            }
        }
    }

    leading.insert(prev);

    ans.iter()
        .map(|(&k, &v)| (k, v, leading.contains(&k)))
        .collect()
}

fn find(
    ptr: usize,
    ans: &mut Vec<u8>,
    digits: &[u8],
    values: &Vec<(char, i64, bool)>,
) -> Option<HashMap<char, u8>> {
    if ptr == values.len() {
        return if cal(ans, values) {
            Some(
                ans.iter()
                    .zip(values.iter())
                    .map(|(&digit, &(ch, _, _))| (ch, digit))
                    .collect(),
            )
        } else {
            None
        };
    }

    for (i, &digit) in digits.iter().enumerate() {
        if digit == 0 && values[ptr].2 {
            continue;
        }

        ans[ptr] = digit as u8;

        let mut digits = digits.to_vec();

        digits.remove(i);

        if let Some(solution) = find(ptr + 1, ans, &digits, values) {
            return Some(solution);
        }
    }

    None
}

fn cal(ans: &[u8], values: &[(char, i64, bool)]) -> bool {
    let mut total = 0;

    for (i, &digit) in ans.iter().enumerate() {
        total += digit as i64 * values[i].1;
    }

    total == 0
}
