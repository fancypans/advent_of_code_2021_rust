use std::fs;

fn main() {
    // --snip--
    let file_path = "test_input";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let split = contents.split("\n");
    let mut first = true;
    let mut prev:u32 = 0;
    let mut is_larger:u32 = 0;
    for s in split {
        if s.is_empty()
        {
            break;
        }
        let f: u32 = s.trim().parse().expect("Wanted a number");
        if first
        {
            first = false;
        }
        else if f < prev
        {
            println!("{}: decreased", f);
        }
        else if f > prev
        {
            println!("{}: increased", f);
            is_larger += 1;
        }
        prev = f;
    }
    println!("number of time depth increased: {}", is_larger);
}
