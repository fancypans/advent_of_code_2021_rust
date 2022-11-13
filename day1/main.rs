use std::fs;

fn main() {
    // let file_path = "test_input";
    let file_path = "puzzle_input";

    let binding = fs::read_to_string(file_path).unwrap_or_default(); //why we need a let binding here?
    let line: Vec<u32> = binding
        .lines()
        .map(|x| x.parse::<u32>().unwrap_or_default())
        .collect();
    //part1(&line);
    println!("part1: {:?}", part1(&line));
    println!("part2: {:?}", part2(&line));
}
fn part1(line: &Vec<u32>) -> usize {
    return line.windows(2).filter(|x| x[0] < x[1]).count();
}
fn part2(line: &Vec<u32>) -> usize {
    return part1(
        &line
            .windows(3)
            .map(|x| x[0] + x[1] + x[2])
            .collect::<Vec<u32>>(),
    );
}
