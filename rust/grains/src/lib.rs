pub fn square(s: u32) -> u64 {
    // 'borrowing' the match case from Wow-BOB-Wow's solution
    // I like branchless programming better, it seems hot.
    // if s < 1 || s > 64 {
    //     panic!("Square must be between 1 and 64");
    // }
    // return 2u64.pow(s - 1);
    match s {
        1..=64 => 2u64.pow(s-1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    // every bit represents a square on the chess board
    // 0 means the square is empty and 1 means it has rice
    // if all the squares have rice, then all the bits are 1
    // this means the number is the max number possible in 64bits
    // I was a nerd and derived this from the qeometric sequence equation
    // this solution seems more understandable.
    return u64::MAX;
}
