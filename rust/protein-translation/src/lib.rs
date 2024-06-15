// RNA: "AUGUUUUCU" -> Codons: "AUG", "UUU", "UCU" -> Protein
//
// Codon 	            Protein
// AUG 	                Methionine
// UUU, UUC 	        Phenylalanine
// UUA, UUG        	    Leucine
// UCU, UCC, UCA, UCG 	Serine
// UAU, UAC 	        Tyrosine
// UGU, UGC 	        Cysteine
// UGG 	                Tryptophan
// UAA, UAG, UGA 	    STOP

// use once_cell::sync::Lazy;
// use std::collections::HashMap;
// use std::sync::Mutex;
// static DB: Lazy<Mutex<HashMap<Vec<String>, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub struct CodonsInfo {
    pairs: Vec<(String, String)>,
    // We fake using 'a here, so the compiler does not complain that
    // "parameter `'a` is never used". Delete when no longer needed.
    // phantom: std::marker::PhantomData<&'a ()>,
}

impl CodonsInfo {
    pub fn name_for(&self, codon: &str) -> Option<&str> {
        for (l, r) in &self.pairs {
            if l == codon {
                return Some(&r);
            }
        }
        None
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&str>> {
        let b = rna.to_string().chars().collect::<Vec<_>>();
        let chunks = b.chunks(3);
        let mut vec = Vec::new();
        for chunk in chunks {
            let slice = chunk.iter().collect::<String>();
            dbg!(&slice);
            if let Some(i) = self.pairs.iter().position(|(l, _)| l == &slice) {
                if vec.len() == 3 || self.pairs[i].1.as_str() == "stop codon" {
                    break;
                }
                vec.push(self.pairs[i].1.as_str());
            } else {
                return None;
            }
        }
        Some(vec)
    }
}

pub fn parse(pairs: Vec<(&str, &str)>) -> CodonsInfo {
    CodonsInfo {
        pairs: pairs
            .iter()
            .map(|(l, r)| (l.to_string(), r.to_string()))
            .collect(),
    }
}
