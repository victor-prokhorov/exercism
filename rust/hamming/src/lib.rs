
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    match s1.len() == s2.len() {
        false => None, 
        true => s1.chars().zip(s2.chars()).filter(|(l, r)| l != r).count().into(),
    }
}
