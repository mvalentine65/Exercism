use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new();
    for i in 1..limit {
        for factor in factors {
            if *factor != 0 && i % factor == 0 {
                multiples.insert(i);
            }
        }
    }
    return multiples.iter().fold(0, | sum, item| sum+item)
}