use core::panic;
use std::cmp;

fn solve(input: &[Vec<char>]) -> i64 {
    // check if just single worksheet column
    if input[input.len() - 1].iter().filter(|&x| *x != ' ').count() == 1 {
        // find the operation
        let op = input[input.len() - 1][0];
        let (operation, start): (fn(i64, i64) -> i64, i64) = match op {
            '*' => (|x, y| x * cmp::max(y, 1), 1),
            '+' => (|x, y| x + y, 0),
            _ => panic!("unknown operation"),
        };
        return (0..input[0].len()).fold(start, |acc, j| {
            operation(
                acc,
                (0..input.len() - 1)
                    .filter(|i| input[*i][j] != ' ')
                    .fold(0, |acc, i: usize| {
                        10 * acc + input[i][j].to_digit(10).unwrap()
                    }) as i64,
            )
        });
    }
    let ind = input[input.len() - 1]
        .iter()
        .skip(1)
        .position(|x| *x != ' ')
        .unwrap() + 1;
    let input1: Vec<Vec<char>> = (0..input.len()).map(|i| input[i][..ind].to_vec()).collect();
    let input2: Vec<Vec<char>> = (0..input.len()).map(|i| input[i][ind..].to_vec()).collect();
    return solve(&input1) + solve(&input2);
}
fn main() {
    let input: Vec<Vec<char>> = include_str!("../input/input.txt")
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();
    let ans = solve(&input);
    println!("{ans}");
}
