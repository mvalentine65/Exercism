pub fn nth(n: u32) -> u32 {
    let mut sieve = vec![(2 as u32)];
    let mut current:f64 = 1.0;
    while sieve.len() as u32 <= n {
        current += 2.0;
        let mut prime:bool = true;
        for i in &sieve {
            if prime == true {
                if (current/(*i as f64)).fract() == 0.0 {
                    prime = false;
               }
            }
        }
        if prime == true{ sieve.push(current as u32); }
    }
    return sieve[sieve.len()-1];
}

    
    