pub fn square_of_sum(n: u32) -> u32 {
    let numbers = 1..=n;
    return u32::pow(numbers.fold(0,|acc, i| acc + i),2);
}

pub fn sum_of_squares(n: u32) -> u32 {
    let numbers = 1..=n;
    return numbers.map(|item| u32::pow(item,2)).fold(0,|acc, item| acc + item);
}

pub fn difference(n: u32) -> u32 {
    return  square_of_sum(n) - sum_of_squares(n);
}
