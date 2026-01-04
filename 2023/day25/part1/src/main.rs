use petgraph::graphmap::UnGraphMap;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
fn main() {
    let edges: Vec<(&str, &str)> = include_str!("../input/input.txt")
        .split("\n")
        .flat_map(|str| {
            let (node, neighbors) = str.split_once(":").unwrap();
            neighbors
                .split_whitespace()
                .map(|x| (node, x))
                .collect::<Vec<(&str, &str)>>()
        })
        .collect();
    let graph: UnGraphMap<&str, ()> = UnGraphMap::from_edges(edges);
    let min_cut = stoer_wagner_min_cut(&graph, |_| Ok::<i32, ()>(1));
    let (_, vec) = min_cut.unwrap().unwrap();
    print!("{}", vec.len() * (graph.node_count() - vec.len()));
}
