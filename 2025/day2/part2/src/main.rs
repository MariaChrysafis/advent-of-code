fn is_valid(num: i64) -> bool {
    let str = num.to_string();
    for length in (1..str.len()).filter(|length| str.len().is_multiple_of(*length)) {
        if (1..str.len() / length).all(|i| str[0..length] == str[i * length..i * length + length])
        {
            return false;
        }
    }
    true
}

fn main() {
    let input = include_str!("../input/sample.txt").replace("\n", "");
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
