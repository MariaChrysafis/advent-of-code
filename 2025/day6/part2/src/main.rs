use core::panic;
use std::cmp;

fn solve(input: &[Vec<char>]) -> i64 {
    let index = input[input.len() - 1]
        .iter()
        .skip(1)
        .position(|x| *x != ' ');
    // check if just single worksheet column
    if index == None {
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
    let ind = index.unwrap() + 1;
    let (input1, input2): (Vec<Vec<char>>, Vec<Vec<char>>) = input
        .iter()
        .map(|vec| {
            let (l, r) = vec.split_at(ind);
            return (l.to_vec(), r.to_vec());
        })
        .unzip();
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
