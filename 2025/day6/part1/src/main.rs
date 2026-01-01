fn main() {
    let input: Vec<Vec<&str>> = include_str!("../input/input.txt")
        .split("\n")
        .map(|x| x.trim().split_whitespace().collect::<Vec<&str>>()).collect();
    let mut ans = 0;
    for i in 0..input[0].len() {
        let op = input[input.len() - 1][i];
        let operator: fn(i64, i64) -> i64 = match op {
            "*" => |x, y| x * y,
            "+" => |x, y| x + y,
            _ => panic!("operator not recognized"),
        };
        let result = (0..input.len()-1).map(|j| input[j][i].parse::<i64>().unwrap()).into_iter().reduce(operator);
        ans += result.unwrap();
    }
    println!("{ans}");
}
