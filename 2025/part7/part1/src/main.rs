use std::collections::HashSet;
fn main() {
    let mut input = include_str!("../input/input.txt")
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>());
    let mut beams: HashSet<i32> = HashSet::from([
        input
            .next()
            .unwrap()
            .iter()
            .position(|x| *x == 'S')
            .unwrap() as i32,
    ]);
    let mut ans = 0;
    for row in input {
        beams = beams
            .iter()
            .flat_map(|j| {
                if row[*j as usize] != '^' {
                    vec![*j]
                } else {
                    ans += 1;
                    vec![j - 1, j + 1]
                }
            })
            .filter(|&x| x >= 0 && x < row.len() as i32)
            .collect();
    }
    println!("{ans}")
}
