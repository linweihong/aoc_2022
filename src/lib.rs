use std::fs;

pub fn get_input(src: &str) -> String {
    let inputs = fs::read_to_string(src).expect("File access error");
    return inputs;
}