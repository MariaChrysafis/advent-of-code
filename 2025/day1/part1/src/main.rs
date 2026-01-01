use std::{vec};

enum Direction {
    Left,
    Right,
}

fn parse_line (line: String) -> (Direction, i32) {
    let (dir, value) = line.split_at(1);
    let value = value.parse::<i32>().expect("valid number");
    let direction = match dir {
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("invalid input")
    };
    return (direction, value);
}

fn parse_input () -> Vec<(Direction, i32)> {
    const PARSED_INPUT: &str = include_str!("../input/input.txt");
    let mut results = vec![];
    for x in PARSED_INPUT.split('\n') {
        results.push(parse_line(x.to_string()));
    }
    return results;
}

fn main() {
    let input = parse_input();
    let mut current = 50;
    let mut ans = 0;
    for x in input {
        match x.0 {
            Direction::Left => {current -= x.1;}
            Direction::Right => {current += x.1}
        }
        if current.rem_euclid(100) == 0 {
            ans += 1;
        }
    }
    println!("{ans}");
}