fn main() {
    let input: Vec<Vec<&str>> = include_str!("../../input/input.txt")
        .split("\n")
        .map(|x: &str| x.split_whitespace().collect())
        .collect();
    let ans: i64 = (0..input[0].len())
        .map(|i| {
            (0..input.len() - 1)
                .map(|j| input[j][i].parse::<i64>().unwrap())
                .reduce(match input[input.len() - 1][i] {
                    "*" => |x, y| x * y,
                    "+" => |x, y| x + y,
                    _ => panic!("operator not recognized"),
                })
                .unwrap()
        })
        .sum();
    println!("{ans}");
}
