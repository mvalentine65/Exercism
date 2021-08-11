use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let valid = String::from("ATCG");
    let mut count = 0;
    if !valid.contains(nucleotide) {
        return Err(nucleotide);
    }
    for character in dna.chars() {
        if !valid.contains(character) {
            return Err(character);
        } else if character == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let valid = String::from("ATCG");
    let mut counts: HashMap<char, usize> = [('A', 0), ('T', 0), ('C', 0), ('G', 0)]
        .iter()
        .cloned()
        .collect();
    for character in dna.chars() {
        if !valid.contains(character) {
            return Err(character);
        } else {
            *counts.get_mut(&character).unwrap() += 1;
        }
    }
    Ok(counts)
}
