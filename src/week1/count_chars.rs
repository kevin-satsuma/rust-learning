// Implement a function count_chars which returns how many times a character c is found in a string s
pub fn main(s: &str, c: char) -> i32 {
    let mut count = 0;
    for v in s.chars() {
        if v == c {
            count += 1;
        }
    }
    count
}