fn main() {
    let input: Vec<Vec<&str>> = include_str!("../input/input.txt")
        .split("\n")
        .map(|x| x.trim().split_whitespace().collect::<Vec<&str>>())
        .collect();
    let mut ans = 0;
    for i in 0..input[0].len() {
        ans += (0..input.len() - 1)
            .map(|j| input[j][i].parse::<i64>().unwrap())
            .into_iter()
            .reduce(match input[input.len() - 1][i] {
                "*" => |x, y| x * y,
                "+" => |x, y| x + y,
                _ => panic!("operator not recognized"),
            })
            .unwrap();
    }
    println!("{ans}");
}
