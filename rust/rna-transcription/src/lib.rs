#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.chars()
            .enumerate()
            .map(|(i, c)| match c {
                'A' | 'C' | 'G' | 'T' => Ok(c),
                _ => Err(i),
            })
            .collect::<Result<String, usize>>()
            .map(Dna)
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => panic!(),
            })
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.chars()
            .enumerate()
            .map(|(i, c)| match c {
                'A' | 'C' | 'G' | 'U' => Ok(c),
                _ => Err(i),
            })
            .collect::<Result<String, usize>>()
            .map(Rna)
    }
}
