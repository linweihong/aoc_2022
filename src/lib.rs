use std::fs;

pub fn get_input(src: &str) -> String {
    let inputs = fs::read_to_string(src).expect("File access error");
    return inputs;
}

pub fn divmod(a: i32, b: i32) -> (i32, i32) {
    return (a / b, a % b)
}