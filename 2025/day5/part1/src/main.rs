fn main() {
    let (inp1, inp2) = include_str!("../input/input.txt")
        .split_once("\n\n")
        .unwrap();
    let numbers: Vec<i64> = inp2
        .split("\n")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let intervals: Vec<(i64, i64)> = inp1
        .split("\n")
        .map(|x| {
            let (l, r) = x.split_once("-").unwrap();
            (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap())
        })
        .collect();
    let ans = numbers
        .iter()
        .filter(|&x| intervals.iter().any(|(l, r)| l <= x && x <= r))
        .count();
    println!("{ans}");
}
