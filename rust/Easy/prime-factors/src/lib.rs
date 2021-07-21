pub fn factors(n: u64) -> Vec<u64> {
    let mut primes:Vec<u64> = Vec::new();
    let mut factor:u64 = 2;
    let mut target:u64 = n.clone();
    while target > 1 {
        while target%factor == 0 {
            primes.push(factor);
            target /= factor;
        }
        factor += 1;
    }
    return primes;
}

