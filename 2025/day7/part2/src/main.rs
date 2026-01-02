use core::panic;
use std::collections::HashMap;
fn main() {
    let mut input = include_str!("../input/input.txt")
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>());
    let mut beams: HashMap<i32, i64> = HashMap::from([(
        input
            .next()
            .unwrap()
            .iter()
            .position(|x| *x == 'S')
            .unwrap() as i32,
        1,
    )]);
    for row in input {
        let mut result = HashMap::new();
        for (i, oc) in beams {
            match row[i as usize] {
                '^' => vec![i - 1, i + 1],
                '.' => vec![i],
                _ => panic!("invalid input"),
            }.iter().for_each(|&ind| *result.entry(ind).or_insert(0) += oc);
        }
        beams = result
            .into_iter()
            .filter(|&(res, _)| res >= 0 && res < row.len() as i32)
            .collect();
    }
    let ans: i64 = beams.values().sum();
    println!("{ans}")
}
