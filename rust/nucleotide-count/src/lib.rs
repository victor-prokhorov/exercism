use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut m = nucleotide_counts(dna)?;
    m.remove(&nucleotide).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut m = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    for ch in dna.chars() {
        m.get_mut(&ch).map(|count| *count += 1).ok_or(ch)?
    }
    Ok(m)
}
