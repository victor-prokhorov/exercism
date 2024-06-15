const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];

pub fn translate(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .map(|s| {
            if s.is_empty() {
                return "".to_string();
            }

            if VOWELS.iter().any(|i| s.starts_with(*i))
                || s.starts_with("xr")
                || s.starts_with("yt")
            {
                return format!("{s}ay");
            }

            let consonant_cluster = s
                .chars()
                .take_while(|i| !VOWELS.contains(i))
                .collect::<String>();

            let t = s[consonant_cluster.len() as usize..].to_string();

            if t.starts_with('u') && consonant_cluster.ends_with('q') {
                let i = &t[1..].to_string();
                return format!("{i}{consonant_cluster}uay");
            }

            let p = consonant_cluster.find('y');
            if let Some(i) = p {
                if i > 0 {
                    let (l, r) = consonant_cluster.split_at(i);
                    return format!("{r}{t}{l}ay");
                }
            }

            format!("{t}{consonant_cluster}ay")
        })
        .collect::<Vec<_>>()
        .join(" ")
}
