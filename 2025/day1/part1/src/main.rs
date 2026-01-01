enum Direction {
    Left,
    Right,
}

fn parse_line (line: &str) -> (Direction, i32) {
    let (dir, value) = line.split_at(1);
    let value = value.parse::<i32>().expect("valid number");
    let direction = match dir {
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("invalid input")
    };
    (direction, value)
}

fn main() {
    let input: Vec<(Direction, i32)> = include_str!("../input/input.txt").lines().map(parse_line).collect();
    let mut current = 50;
    let mut ans = 0;
    for (dir, amount) in input {
        current += match dir {
            Direction::Left => -amount,
            Direction::Right => amount,
        };
        if current.rem_euclid(100) == 0 {
            ans += 1;
        }
    }
    println!("{ans}");
}