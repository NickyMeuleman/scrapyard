use std::collections::HashMap;

const VALID: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID.contains(&nucleotide) {
        return Err(nucleotide);
    }
    match dna.chars().find(|c| !VALID.contains(c)) {
        Some(c) => Err(c),
        None => Ok(dna.matches(nucleotide).count()),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    VALID
        .iter()
        .map(|&c| {
            let num = count(c, dna)?;
            Ok((c, num))
        })
        .collect()
}

// version below doesn't use `count()`, preventing multiple iterations over the string
// pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
//     let options: HashMap<char, usize> = VALID.iter().map(|&n| (n, 0)).collect();
//     dna.chars().fold(Ok(options), |acc, c| {
//         let mut map = acc?;
//         map.get_mut(&c).map(|count| *count += 1).ok_or(c)?;
//         Ok(map)
//     })
// }

// jtmueller
// pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
//     let mut counts = nucleotide_counts(dna)?;
//     counts.remove(&nucleotide).ok_or(nucleotide)
// }

// pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
//     let mut counts: HashMap<char, usize> = ['A', 'C', 'G', 'T'].iter().map(|n| (*n, 0)).collect();

//     for c in dna.chars() {
//         counts.get_mut(&c).map(|count| *count += 1).ok_or(c)?
//     }

//     Ok(counts)
// }