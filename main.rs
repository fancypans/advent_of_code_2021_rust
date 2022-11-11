use std::env;
use std::fs;

fn main() {
    // --snip--
    let file_path = "test_input";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
