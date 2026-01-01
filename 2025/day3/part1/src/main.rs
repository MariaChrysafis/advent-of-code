use std::cmp;

fn max_joltage(str: &str) -> u32 {
    let digits: Vec<u32> = str.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut mx = 0;
    for i in 0..digits.len() {
        for j in 0..digits.len() {
            if i < j {
                mx = cmp::max(mx, 10 * digits[i] + digits[j]);
            }
        }
    }
    return mx;
}

fn main() {
    let ans: u32 = include_str!("../input/input.txt")
        .lines()
        .map(max_joltage)
        .sum::<u32>();
    println!("{ans}");
}
