pub fn main(s: String) -> String{
    let mut start = 0;
    for c in s.chars() {
        if char::is_whitespace(c) {
            start += 1;
        } else {
            break
        }
    }
    let mut end = s.len() - 1;
    for c in s.chars().rev() {
        if char::is_whitespace(c) {
            end -= 1;
        } else {
            break
        }
    }
    return s[start..=end].to_string()
}