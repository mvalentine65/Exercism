pub fn bottles(n: u32) -> String {
    if n == 0 {
        return String::from("No more bottles");
    } else if n == 1 {
        return String::from("1 bottle");
    } else {
        return format!("{} bottles", n);
    }
}

pub fn verse(n: u32) -> String {
    let mut song: Vec<String> = vec![format!(
        "{} of beer on the wall, {} of beer.\n",
        bottles(n),
        bottles(n).to_lowercase()
    )];
    if n > 1 {
        song.push(format!(
            "Take one down and pass it around, {} of beer on the wall.\n",
            bottles(n - 1)
        ));
    } else if n == 1 {
        song.push(format!(
            "Take it down and pass it around, {} of beer on the wall.\n",
            bottles(n - 1).to_lowercase()
        ));
    } else {
        song.push(String::from(
            "Go to the store and buy some more, 99 bottles of beer on the wall.\n",
        ));
    }
    song.join("")
}

pub fn sing(start: u32, end: u32) -> String {
    let mut verses: Vec<String> = Vec::new();
    let mut count = start;
    while count >= end {
        verses.push(verse(count));
        if count == 0 {
            break;
        };
        count -= 1;
    }
    verses.join("\n")
}
