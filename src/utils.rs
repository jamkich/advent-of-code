use std::fs;

#[allow(dead_code)]
pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Cant read file")
}
