const VALID_DNA: [char; 4] = ['A', 'C', 'G', 'T'];
const VALID_RNA: [char; 4] = ['A', 'C', 'G', 'U'];

// private struct fields cannot be accessed/modified directly outside of this module
#[derive(Debug, PartialEq)]
pub struct Dna {
    sequence: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    sequence: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.chars().position(|c| !VALID_DNA.contains(&c)) {
            Some(idx) => Err(idx),
            None => Ok(Self {
                sequence: dna.to_string(),
            }),
        }
    }

    pub fn into_rna(self) -> Rna {
        let rna = self
            .sequence
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect();
        Rna { sequence: rna }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna.chars().position(|c| !VALID_RNA.contains(&c)) {
            Some(idx) => Err(idx),
            None => Ok(Self {
                sequence: rna.to_string(),
            }),
        }
    }
}
