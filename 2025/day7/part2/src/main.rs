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
            if row[i as usize] != '^' {
                *result.entry(i).or_insert(0) += oc;
            } else {
                *result.entry(i - 1).or_insert(0) += oc;
                *result.entry(i + 1).or_insert(0) += oc;
            }
        }
        beams = result.into_iter().filter(|&(res, _)| res >= 0 && res < row.len() as i32).collect();
    }
    let ans: i64 = beams.iter().map(|(_, oc)| oc).sum();
    println!("{ans}")
}
