use core::panic;
use std::collections::HashMap;
fn main() {
    let mut lines = include_str!("../input/input.txt").lines();
    let start = lines.next().unwrap().find('S').unwrap();
    let mut beams = HashMap::from([(start, 1)]);
    for row in lines
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
    {
        let mut result = HashMap::new();
        for (i, oc) in beams {
            match row[i as usize] {
                '^' => vec![i - 1, i + 1],
                '.' => vec![i],
                _ => panic!("invalid input"),
            }
            .iter()
            .for_each(|&ind| *result.entry(ind).or_insert(0) += oc);
        }
        result.retain(|&i, _| (0..row.len()).contains(&i));
        beams = result;
    }
    let ans: i64 = beams.values().sum();
    println!("{ans}")
}
