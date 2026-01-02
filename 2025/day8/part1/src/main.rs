struct Graph {
    adj: Vec<Vec<usize>>,
}
impl Graph {
    fn dfs(&mut self, vis: &mut Vec<usize>, node: usize, cntr: usize) -> usize {
        if vis[node] != 0 {
            return 0;
        }
        vis[node] = cntr;
        let mut ans = 1;
        for x in self.adj[node].clone() {
            ans += self.dfs(vis, x, cntr);
        }
        ans
    }
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        self.adj[v].push(u);
    }
    pub fn components(&mut self) -> Vec<usize> {
        let mut vis = vec![0; self.adj.len()];
        let mut sizes = Vec::new();
        for i in 0..self.adj.len() {
            sizes.push(self.dfs(&mut vis, i, i + 1));
        }
        sizes
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
    let mut edges: Vec<(usize, usize)> = vec![];
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            edges.push((i, j));
        }
    }
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
