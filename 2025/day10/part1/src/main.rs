use std::collections::HashMap;
use std::collections::VecDeque;
fn solve(state: usize, vec: &[usize]) -> i32 {
    let mut dp: HashMap<usize, i32> = HashMap::new();
    let mut queue: VecDeque<(usize, i32)> = VecDeque::from(vec![(state, 0)]);
    while let Some((node, dist)) = queue.pop_front() {
        if dp.contains_key(&node) {
            continue;
        }
        dp.insert(node, dist);
        vec.iter()
            .for_each(|x| queue.push_back((node ^ x, dist + 1)));
    }
    *dp.get(&0).unwrap()
}
fn main() {
    let ans: i32 = include_str!("../input/sample.txt")
        .split("\n")
        .map(|line| {
            let x: Vec<&str> = line.split(" ").collect();
            let mask: usize = x[0][1..x[0].len() - 1]
                .char_indices()
                .map(|(i, c)| if c == '#' { 1 << i } else { 0 })
                .sum();
            let positions: Vec<usize> = x[1..x.len() - 1]
                .to_vec()
                .iter()
                .map(|str| {
                    str[1..str.len() - 1]
                        .split(',')
                        .map(|x| 1 << x.parse::<usize>().unwrap())
                        .sum()
                })
                .collect();
            solve(mask, &positions)
        })
        .sum();
    println!("{}", ans);
}
