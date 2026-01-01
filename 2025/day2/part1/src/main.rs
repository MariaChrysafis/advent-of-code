fn is_valid (i: i64) -> bool {
    let str = i.to_string();
    let len = str.len();
    return str[0..len/2] != str[len/2..len];
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
        for i in left..right + 1 {
            if !is_valid(i) {
                count += i;
            }
        }
    }
    println!("{count}")
}
