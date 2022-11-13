use std::fs;
use std::str::FromStr;

fn main() {
    let file_path = "test_input";
    //let file_path = "puzzle_input";

    let binding = fs::read_to_string(file_path).unwrap_or_default(); //why we need a let binding here?

    let line = binding.lines().collect();
}

#[derive(Debug, PartialEq)]
enum Direction {
    Down,
    Up,
    Forward,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Movement {
    direction: Direction,
    stepsize: i32,
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(' ').unwrap();

        Ok(Movement {
            direction: x.parse::<Direction>().unwrap(),
            stepsize: y.parse::<i32>().unwrap(),
        })
    }
}
