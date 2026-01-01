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
    let moves: Vec<(Direction, i32)> = include_str!("../input/input.txt")
        .lines()
        .map(parse_line)
        .collect();
    let mut current_position: i32 = 50;
    let mut cnt = 0;
    for (direction, delta) in moves {
        let mut new_position: i32 = current_position;
        match direction {
            Direction::Left => {
                new_position -= delta;
                if new_position <= 0 {
                    cnt += -new_position / 100 + (current_position + 99) / 100
                }
            }
            Direction::Right => {
                new_position += delta;
                cnt += new_position / 100 - current_position / 100
            }
        }
        current_position = new_position.rem_euclid(100);
    }
    println!("{cnt}");
}
