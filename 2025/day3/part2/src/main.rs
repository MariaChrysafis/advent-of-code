use std::cmp;

fn max_joltage(str: &str, batteries: usize) -> String {
    let mut dp: Vec<Vec<String>> = vec![vec![String::new(); batteries + 1]; str.len() + 1];
    for ind in 0..str.len() + 1 {
        for battery in 0..(batteries + 1) as usize {
            // str[0...i], using battery many batteries
            if battery == 0 || ind == 0 {
                dp[ind][battery] = String::new();
            } else {
                let mut prev = String::from(dp[ind - 1][battery - 1].clone());
                prev.push(str.chars().nth(ind - 1).unwrap());
                dp[ind][battery] = cmp::max(dp[ind - 1][battery].clone(), prev);
            }
        }
    }
    return dp[str.len()][batteries].clone();
}

fn main() {
    let joltages: Vec<String> = include_str!("../input/input.txt")
        .lines()
        .map(|str| max_joltage(str, 12)).collect();
    let ans: i64 = joltages.iter().map(|x| x.parse::<i64>().unwrap()).sum::<i64>();
    println!("{ans}");
}
