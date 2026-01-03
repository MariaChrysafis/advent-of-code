use std::collections::HashMap;
type Node = (String, [bool; 2]);
struct Graph {
    adj: HashMap<Node, Vec<Node>>,
}
impl Graph {
    pub fn dfs(&self, node: &Node, end: &Node, dp: &mut HashMap<Node, i64>) -> i64 {
        if dp.contains_key(node) {
            return *dp.get(node).unwrap();
        }
        if node == end {
            return 1;
        }
        let result: i64 = self
            .adj
            .get(node)
            .map_or(&vec![], |v| v)
            .iter()
            .map(|next| self.dfs(next, end, dp))
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
    let mut adjn: HashMap<Node, Vec<Node>> = HashMap::new();
    for (node, neighbors) in adj {
        for flags in [[true, false], [true, true], [false, true], [false, false]] {
            for n in &neighbors {
                adjn.entry((node.clone(), flags)).or_default().push((
                    n.to_string(),
                    [flags[0] || n == "dac", flags[1] || n == "fft"],
                ));
            }
        }
    }
    let graph = Graph { adj: adjn };
    let start = &("svr".to_string(), [false, false]);
    let end = &("out".to_string(), [true, true]);
    let mut dp: HashMap<Node, i64> = HashMap::new();
    println!("{}", graph.dfs(start, end, &mut dp))
}
