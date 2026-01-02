use core::panic;

fn solve(mut input: Vec<Vec<char>>) -> i64 {
    // check if just single worksheet column
    if input[input.len() - 1].iter().filter(|&x| *x != ' ').count() == 1 {
        // find the operation
        let op = input[input.len() - 1][0];
        let (operation, mut ans): (fn(i64, i64) -> i64, i64) = match op {
            '*' => (|x, y| x * y, 1),
            '+' => (|x, y| x + y, 0),
            _ => panic!("unknown operation"),
        };
        for j in 0..input[0].len() {
            let mut res = 0;
            for i in 0..input.len() - 1 {
                if input[i][j] == ' ' {
                    continue;
                }
                res = 10 * res + input[i][j].to_digit(10).unwrap();
            }
            if res != 0 {
                ans = operation(ans, res as i64);
            }
        }
        return ans;
    }
    let ind = input[input.len() - 1]
        .iter()
        .enumerate()
        .filter(|&(_, c)| *c != ' ')
        .map(|x| x.0).collect::<Vec<usize>>()[1];
    let input1: Vec<Vec<char>> = (0..input.len()).map(|i| input[i][..ind].to_vec()).collect();
    let input2: Vec<Vec<char>> = (0..input.len()).map(|i| input[i][ind..].to_vec()).collect();
    return solve(input1) + solve(input2);
}
fn main() {
    let input: Vec<Vec<char>> = include_str!("../input/input.txt")
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();
    let ans = solve(input);
    println!("{ans}");
}
