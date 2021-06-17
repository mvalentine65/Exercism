pub fn reply(message: &str) -> &str {
    match message.trim() {
        x if x.is_empty() => "Fine. Be that way!",
        x if x.to_uppercase() == x
            && !(x.ends_with('?'))
            && x.matches(char::is_alphabetic).count() > 0 =>
        {
            "Whoa, chill out!"
        }
        x if x.to_uppercase() == x
            && x.ends_with('?')
            && x.matches(char::is_alphabetic).count() > 0 =>
        {
            "Calm down, I know what I'm doing!"
        }
        x if x.ends_with('?') => "Sure.",
        _ => "Whatever.",
    }
}
