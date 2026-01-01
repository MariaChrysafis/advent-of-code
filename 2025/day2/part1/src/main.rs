fn is_valid(i: i64) -> bool {
    let str = i.to_string();
    let len = str.len();
    return str[..len / 2] != str[len / 2..];
}

fn main() {
    let input = include_str!("../input/input.txt").replace("\n", "");
    let moves: Vec<&str> = input.split(",").collect();
    let mut count = 0;
    for range in moves {
        let (left, right) = range
            .split_once("-")
            .map(|(l, r)| (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap()))
            .expect("only two pairs");
        count += (left..=right).filter(|&i| !is_valid(i)).sum::<i64>();
    }
    println!("{count}")
}
