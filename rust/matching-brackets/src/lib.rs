use std::collections::HashMap;

// turns out this has to deal with stings containing numbers too

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::<char>::new();
    let opener_needed: HashMap<char, char> = [('}', '{'), (']', '['), (')', '(')]
        .iter()
        .cloned()
        .collect();
    for c in string.chars() {
        if String::from("{{}}[]()").contains(c) {
            match opener_needed.get(&c) {
                Some(&closer) => {
                    if stack.is_empty() {
                        return false;
                    } else if stack.pop().unwrap() != closer {
                        return false;
                    }
                }
                None => stack.push(c),
            }
        }
    }
    return stack.is_empty();
}
