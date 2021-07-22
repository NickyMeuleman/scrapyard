use std::collections::HashMap;
use std::str;

pub struct CodonsInfo<'a> {
    map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    fn new(pairs: Vec<(&'a str, &'a str)>) -> Self {
        Self {
            map: pairs.into_iter().collect(),
        }
    }

    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.map.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        // a chunks method directly on a string sure would be nice
        rna.as_bytes()
            .chunks(3)
            .map(|bytes| {
                let codon = str::from_utf8(bytes).ok()?;
                self.name_for(codon)
            })
            .take_while(|&codon| codon != Some("stop codon"))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo::new(pairs)
}
