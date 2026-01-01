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
    return (direction, value);
}

fn parse_input () -> Vec<(Direction, i32)> {
    const INPUT: &str = include_str!("../input/input.txt");
    INPUT.lines().map(parse_line).collect()
}

fn main() {
    let input = parse_input();
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