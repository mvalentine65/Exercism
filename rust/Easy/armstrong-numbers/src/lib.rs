pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = Vec::<u32>::new();
    let mut working: u32 = num;
    while working > 0 {
        digits.push(working % 10);
        working /= 10;
    }
    let sum: u32 = digits
        .iter()
        .map(|item| item.pow(digits.len() as u32))
        .sum();
    return num == sum;
}
