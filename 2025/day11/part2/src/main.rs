use std::collections::HashMap;
type Node = (String, [bool; 2]);

const TARGETS: [&str; 2] = ["dac", "fft"];

fn dfs(adj: &HashMap<String, Vec<String>>, node: &Node, dp: &mut HashMap<Node, i64>) -> i64 {
    if let Some(&v) = dp.get(node) {
        return v;
    }
    let result: i64 = adj
        .get(&node.0)
        .map(|v| v.as_slice())
        .unwrap_or(&[])
        .iter()
        .map(|n| {
            let next = (
                n.clone(),
                [node.1[0] || n == TARGETS[0], node.1[1] || n == TARGETS[1]],
            );
            dfs(adj, &next, dp)
        })
        .sum();
    dp.insert(node.clone(), result);
    result
}

fn main() {
    let adj: HashMap<String, Vec<String>> = include_str!("../input/input.txt")
        .lines()
        .map(|line| {
            let (node, neighbors) = line.split_once(':').unwrap();
            (
                node.to_string(),
                neighbors
                    .split_whitespace()
                    .map(|x| x.to_string())
                    .collect(),
            )
        })
        .collect();
    let start = &("svr".to_string(), [false, false]);
    let end = &("out".to_string(), [true, true]);
    println!(
        "{}",
        dfs(&adj, start, &mut HashMap::from([(end.clone(), 1i64)]))
    )
}
