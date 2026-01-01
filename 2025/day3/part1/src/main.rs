use std::cmp;

fn main() {
    let input = include_str!("../input/input.txt").split("\n");
    let mut ans = 0;
    for str in input {
        let digits: Vec<u32> = str.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut mx = 0;
        for i in 0..digits.len() {
            for j in 0..digits.len() {
                if i < j {
                    mx = cmp::max(mx, 10 * digits[i] + digits[j]);
                }
            }
        }
        ans += mx;
    }
    println!("{ans}");
}
