use std::collections::{self, HashSet};
fn main() {
    let mut input = include_str!("../input/input.txt")
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>());
    let mut beams: collections::HashSet<i32> = collections::HashSet::from([
        input
            .next()
            .unwrap()
            .iter()
            .position(|x| *x == 'S')
            .unwrap() as i32,
    ]);
    let mut ans = 0;
    for row in input {
        let new_beams: HashSet<i32> = beams
            .iter()
            .flat_map(|j| {
                if row[*j as usize] != '^' {
                    return vec![*j];
                } else {
                    ans += 1;
                    return vec![j - 1, j + 1];
                }
            })
            .filter(|&x| x >= 0 && x < row.len() as i32)
            .collect();
        beams = new_beams.into_iter().collect();
    }
    println!("{ans}")
}
