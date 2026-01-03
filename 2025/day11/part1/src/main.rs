use std::collections::HashMap;
fn dfs(x: &str, adj: &Vec<(&str, Vec<&str>)>, cnt: &mut HashMap<String, i32>) {
    *cnt.entry(x.to_string()).or_insert(0) += 1;
    for (node, neighbors) in adj {
        if *node == x {
            for neighbor in neighbors {
                dfs(neighbor, adj, cnt);
            }
        }
    }
}

fn main() {
    let adj: Vec<(&str, Vec<&str>)> = include_str!("../input/input.txt")
        .lines()
        .map(|str| {
            let (node, neighbors) = str.split_once(':').unwrap();
            (node, neighbors.split_whitespace().collect())
        })
        .collect();
    const START: &str = "you";
    const END: &str = "out";
    let mut cnt: HashMap<String, i32> = HashMap::new();
    dfs(START, &adj, &mut cnt);
    println!("{}", cnt.get(END).unwrap());
}
