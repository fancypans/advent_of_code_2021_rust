use std::fs;

fn main() {
    // --snip--
    let file_path = "test_input";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let split = contents.split("\n");
    let mut first = true;
    let mut prev:u8 = 0;
    for s in split {
        println!("number: {}",s);
        let f: u8 = s.trim().parse().expect("Wanted a number");
        if first
        {
            prev = f;
            first = false;
        }
        else if f < prev
        {
            println!("s: decreased");
        }
        else if f > prev
        {
            println!("s: increased");
        }
    }
}
