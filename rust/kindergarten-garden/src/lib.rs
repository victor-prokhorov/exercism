const LOOKUP: [u8; 4] = [b'C', b'G', b'R', b'V'];
const PLANT_NAMES: [&str; 4] = ["clover", "grass", "radishes", "violets"];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut result = Vec::with_capacity(4);
    let n = 2 * (student.as_bytes()[0] - b'A') as usize;
    for line in diagram.lines() {
        for c in line.as_bytes()[n..n + 2].iter() {
            let i = LOOKUP.iter().position(|x| x == c).unwrap();
            result.push(PLANT_NAMES[i]);
        }
    }
    result
}
