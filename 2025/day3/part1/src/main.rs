use std::cmp;

fn main() {
    let input = include_str!("../input/input.txt").split("\n");
    let mut ans = 0;
    for str in input {
        let mut mx = 0;
        for i in 0..str.len() {
            for j in 0..str.len() {
                let digit1 = str.chars().nth(i).unwrap().to_digit(10).unwrap();
                let digit2 = str.chars().nth(j).unwrap().to_digit(10).unwrap();
                if i < j {
                    mx = cmp::max(mx, 10 * digit1 + digit2);
                }
            }
        }
        ans += mx;
    }
    println!("{ans}");
}
