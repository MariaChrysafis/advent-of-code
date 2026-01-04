use petgraph::graph::UnGraph;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use std::collections::{HashMap, HashSet};
fn main() {
    let edges: Vec<(&str, &str)> = include_str!("../input/input.txt")
        .split("\n")
        .flat_map(|str| {
            let (node, neighbors) = str.split_once(":").unwrap();
            let mut arr = vec![];
            for x in neighbors.split_whitespace() {
                arr.push((node, x));
                //println!("{x} {node}");
            }
            arr
        })
        .collect();
    let nodes: HashSet<&str> = edges
        .iter()
        .flat_map(|&edge| vec![edge.0, edge.1])
        .collect();
    // map each node to an integer
    let to_int: HashMap<&str, u32> = nodes
        .iter()
        .enumerate()
        .map(|(x, y)| (*y, x as u32))
        .collect();
    let new_edges = edges.iter().map(|(x, y)| (to_int[*x], to_int[*y]));
    let graph: UnGraph<usize, ()> = UnGraph::from_edges(new_edges);
    let min_cut = stoer_wagner_min_cut(&graph, |_| Ok::<i32, ()>(1));
    let (_, vec) = min_cut.unwrap().unwrap();
    print!("{}", vec.len() * (graph.node_count() - vec.len()));
}
