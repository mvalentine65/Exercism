pub fn build_proverb(list: &[&str]) -> String {
    let mut answer: String = String::new();
    let mut i:usize = 0;
    if list.len() > 0 {
        while i < list.len()-1 {
            answer.push_str(&format!("For want of a {} the {} was lost.\n",list[i],list[i+1]).to_string());
            i += 1;
        }
        answer.push_str(&format!("And all for the want of a {}.",list[0]).to_string());
    }
    return answer;
}
