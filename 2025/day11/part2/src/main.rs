use std::collections::HashMap;
type Node = (String, [bool; 2]);
struct Graph {
    adj: HashMap<String, Vec<String>>,
}
impl Graph {
    pub fn dfs(&self, node: &Node, dp: &mut HashMap<Node, i64>) -> i64 {
        if dp.contains_key(node) {
            return *dp.get(node).unwrap();
        }
        let result: i64 = self
            .adj
            .get(&node.0)
            .map_or(&vec![], |v| v)
            .iter()
            .map(|n| {
                let next = (
                    n.clone(),
                    [node.1[0] || n == "dac", node.1[1] || n == "fft"],
                );
                self.dfs(&next, dp)
            })
            .sum();
        dp.insert(node.clone(), result);
        result
    }
}
fn main() {
    let adj: HashMap<String, Vec<String>> = include_str!("../input/input.txt")
        .lines()
        .map(|str| {
            let (node, neighbors) = str.split_once(':').unwrap();
            (
                node.to_string(),
                neighbors
                    .split_whitespace()
                    .map(|x| x.to_string())
                    .collect(),
            )
        })
        .collect();
    let graph = Graph { adj };
    let start = &("svr".to_string(), [false, false]);
    let end = &("out".to_string(), [true, true]);
    println!(
        "{}",
        graph.dfs(start, &mut HashMap::from([(end.clone(), 1i64)]))
    )
}
