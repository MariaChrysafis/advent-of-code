enum Direction {
    Left,
    Right,
}

fn parse_line(line: &str) -> (Direction, i32) {
    let (dir, value) = line.split_at(1);
    let delta = value.parse::<i32>().expect("valid number");
    let direction = match dir {
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("invalid input"),
    };
    (direction, delta)
}

fn main() {
    let moves: Vec<(Direction, i32)> = include_str!("../../input/input.txt")
        .lines()
        .map(parse_line)
        .collect();
    let mut position = 50;
    let mut count = 0;
    for (direction, delta) in moves {
        position += match direction {
            Direction::Left => -delta,
            Direction::Right => delta,
        };
        if position.rem_euclid(100) == 0 {
            count += 1;
        }
    }
    println!("{count}");
}
