use std::cmp;

fn max_joltage(str: &str, batteries: usize) -> i64 {
    let digits: Vec<u32> = str.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut dp: Vec<Vec<i64>> = vec![vec![0; batteries + 1]; str.len() + 1];
    for ind in 1..=str.len() {
        for battery in 1..=batteries {
            dp[ind][battery] = cmp::max(
                dp[ind - 1][battery],
                10 * dp[ind - 1][battery - 1] + digits[ind - 1] as i64,
            );
        }
    }
    return dp[str.len()][batteries];
}

fn main() {
    let ans: i64 = include_str!("../input/input.txt")
        .lines()
        .map(|str| max_joltage(str, 12))
        .sum();
    println!("{ans}");
}
