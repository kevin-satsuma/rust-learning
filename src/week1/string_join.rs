// Join a vector of strings into a single string.
pub fn main(v: Vec<String>) -> String {
    let mut result = "".to_owned();

    for s in v.iter() {
        result = result + s
    }

    result
}