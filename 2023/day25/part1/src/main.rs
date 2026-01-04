// use petgraph::graph;
use petgraph::Undirected;
use petgraph::algo::bridges::bridges;
use petgraph::graph::UnGraph;
use petgraph::graphmap::UnGraphMap;
use petgraph::unionfind;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use std::collections::HashMap;
fn to_int_graph(graph: &UnGraphMap<&str, ()>) -> UnGraph<usize, ()> {
    let hashmap: HashMap<&str, u32> = graph
        .nodes()
        .enumerate()
        .map(|(x, y)| (y, x as u32))
        .collect();
    let mut edges: Vec<(u32, u32, ())> = vec![];
    for edge in graph.edge_references() {
        edges.push((hashmap[edge.0], hashmap[edge.1], ()));
    }
    UnGraph::from_edges(edges)
}
fn component_sizes(graph: UnGraph<usize, ()>) -> Vec<u32> {
    let mut uf = unionfind::UnionFind::new(graph.node_count());
    for edge in graph.edge_references() {
        uf.union(edge.source(), edge.target());
    }
    let mut component_sizes = HashMap::new();
    for (node, _) in graph.node_references() {
        *component_sizes.entry(uf.find(node)).or_insert(0) += 1;
    }
    //println!("{}", graph.node_references().len());
    component_sizes.values().copied().collect()
}
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
    //let g = UnGraphMap::<&str, ()>::from_edges(&edges);
    for (i, edge1) in edges.iter().enumerate() {
        for (j, edge2) in edges.iter().enumerate() {
            if i == j {
                continue;
            }
            if edge1.1 != edge2.1 && edge1.0 != edge2.1 && edge1.0 != edge2.0 && edge1.1 != edge2.0
            {
                continue;
            }
            println!("{} {}", i, j);
            let new_graph = to_int_graph(&UnGraphMap::<&str, ()>::from_edges(
                edges
                    .iter()
                    .enumerate()
                    .filter(|&(ind, _)| ind != i && ind != j)
                    .map(|(_, &val)| val)
                    .collect::<Vec<(&str, &str)>>(),
            ));
            let bridges = bridges(&new_graph).next();
            if let Some(bridge) = bridges {
                let mut graph_without_bridge = new_graph.clone();
                graph_without_bridge.remove_edge(bridge.id());
                assert!(graph_without_bridge.edge_count() == new_graph.edge_count() - 1);
                println!("{} {} {} {}", edge1.0, edge1.1, edge2.0, edge2.1);
                println!(
                    "{}",
                    component_sizes(graph_without_bridge)
                        .iter()
                        .product::<u32>()
                );
                return;
            }

            // remove edges1 and edges2 from edges
        }
    }
    // println!("{}", adj.len());
    // for (_, value) in adj.iter() {
    //     for v in value {
    //         print!("{v} ");
    //     }

    //     println!();
    // }
    //println!("{}", moves);
}
