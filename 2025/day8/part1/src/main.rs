struct Graph {
    adj: Vec<Vec<usize>>,
}
impl Graph {
    fn dfs(&self, vis: &mut Vec<bool>, node: usize) -> usize {
        if vis[node] {
            return 0;
        }
        vis[node] = true;
        self.adj[node]
            .iter()
            .copied()
            .map(|x| self.dfs(vis, x))
            .sum::<usize>()
            + 1
    }
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        self.adj[v].push(u);
    }
    pub fn components(&mut self) -> Vec<usize> {
        let mut vis = vec![false; self.adj.len()];
        (0..self.adj.len()).map(|i| self.dfs(&mut vis, i)).collect()
    }
    pub fn new(n: usize) -> Graph {
        Graph {
            adj: vec![Vec::new(); n],
        }
    }
}
fn distance(pos1: &[i64], pos2: &[i64]) -> i64 {
    (0..pos1.len()).map(|i| (pos1[i] - pos2[i]).pow(2)).sum()
}
fn main() {
    let positions: Vec<Vec<i64>> = include_str!("../input/sample.txt")
        .lines()
        .map(|x| x.split(',').map(|x| x.parse::<i64>().unwrap()).collect())
        .collect();
    let mut edges: Vec<(usize, usize)> = (0..positions.len())
        .flat_map(|i| (i + 1..positions.len()).map(move |j| (i, j)))
        .collect();
    edges.sort_by_key(|&(i, j)| distance(&positions[j], &positions[i]));
    const SIZE: usize = 1000;
    let mut graph = Graph::new(positions.len());
    edges
        .iter()
        .take(SIZE)
        .for_each(|edge| graph.add_edge(edge.0, edge.1));
    let mut components = graph.components();
    components.sort();
    components.reverse();
    let ans = components[0] * components[1] * components[2];
    println!("{ans}");
}
