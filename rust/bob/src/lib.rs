pub fn reply(message: &str) -> &str {
    //    unimplemented!("have Bob reply to the incoming message: {}", message)
    // if all upper question:   'Calm down, I know what I'm doing!'
    // if all upper statement:  'Whoa, chill out!'
    // if regular question:     'Sure.'
    // if regular statement:    'Whatever.'
    // if all whitespace:       'Fine. Be that way!'
    let input: String = message
        .chars()
        .filter(|char| !char.is_whitespace())
        .collect::<String>();
}
