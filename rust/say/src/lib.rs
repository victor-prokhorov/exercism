#[must_use]
pub fn encode(n: u64) -> String {
    encode_whole(n)
}

#[must_use]
pub fn encode_whole(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    encode_natural(n)
}

const PRONUNCIATION_VALUE: [(&str, u64); 7] = [
    ("quintillion", 1_000_000_000_000_000_000),
    ("quadrillion", 1_000_000_000_000_000),
    ("trillion", 1_000_000_000_000),
    ("billion", 1_000_000_000),
    ("million", 1_000_000),
    ("thousand", 1_000),
    ("hundred", 100),
];

#[must_use]
pub fn encode_natural(mut n: u64) -> String {
    match n {
        0 => String::new(),
        1 => "one".into(),
        2 => "two".into(),
        3 => "three".into(),
        4 => "four".into(),
        5 => "five".into(),
        6 => "six".into(),
        7 => "seven".into(),
        8 => "eight".into(),
        9 => "nine".into(),
        10 => "ten".into(),
        11 => "eleven".into(),
        12 => "twelve".into(),
        13 => "thirteen".into(),
        14 => "fourteen".into(),
        15 => "fifteen".into(),
        16 => "sixteen".into(),
        17 => "seventeen".into(),
        18 => "eighteen".into(),
        19 => "nineteen".into(),
        20 => "twenty".into(),
        30 => "thirty".into(),
        40 => "forty".into(),
        50 => "fifty".into(),
        60 => "sixty".into(),
        70 => "seventy".into(),
        80 => "eighty".into(),
        90 => "ninety".into(),
        21..=99 => {
            format!("{}-{}", encode_natural(n - n % 10), encode_natural(n % 10))
        }
        100..=u64::MAX => {
            let mut result = String::new();
            for (pronunciation, value) in PRONUNCIATION_VALUE {
                if n >= value && n % value == 0 {
                    return format!("{} {}", encode_natural(n / value), pronunciation);
                } else if n > value {
                    result.push_str(&format!("{} {} ", encode_natural(n / value), pronunciation));
                    n %= value;
                }
            }
            result.push_str(&encode_natural(n));
            result
        }
    }
}
