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

fn reaches_zero (current_position: i32, delta: i32, direction: &Direction) -> (i32, i32) {
    let mut new_position: i32 = current_position;
    match direction {
        Direction::Left => {
            new_position -= delta;
            if new_position > 0 {
                return (0, new_position);
            }
            return (-new_position/100 + (current_position != 0) as i32, new_position);
        }
        Direction::Right => {
            new_position += delta;
            return (new_position/100 - current_position/100, new_position);
        }
    }
}

fn main() {
    let moves: Vec<(Direction, i32)> = include_str!("../input/input.txt")
        .lines()
        .map(parse_line)
        .collect();
    let mut current_position: i32 = 50;
    let mut cnt = 0;
    for (direction, delta) in moves {
        let result = reaches_zero(current_position.rem_euclid(100i32), delta, &direction);
        cnt += result.0;
        current_position = result.1.rem_euclid(100);
    }
    println!("{cnt}");
}
