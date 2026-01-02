struct Graph {
    adj: Vec<Vec<usize>>,
}
impl Graph {
    fn dfs(&mut self, vis: &mut Vec<usize>, sizes: &mut Vec<usize>, node: usize, cntr: usize) {
        if vis[node] != 0 {
            return;
        }
        *sizes.last_mut().unwrap() += 1;
        vis[node] = cntr;
        for x in self.adj[node].clone() {
            self.dfs(vis, sizes, x, cntr);
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        self.adj[v].push(u);
    }
    pub fn components(&mut self) -> Vec<usize> {
        let mut vis = vec![0; self.adj.len()];
        let mut sizes = Vec::new();
        for i in 0..self.adj.len() {
            sizes.push(0);
            self.dfs(&mut vis, &mut sizes, i, i + 1);
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
    let positions = include_str!("../input/sample.txt")
        .lines()
        .map(|x| {
            x.split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    let mut distances: Vec<(usize, usize)> = vec![];
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            distances.push((i, j));
        }
    }
    distances.sort_by(|edge1, edge2| {
        distance(&positions[edge1.0], &positions[edge1.1])
            .cmp(&distance(&positions[edge2.0], &positions[edge2.1]))
    });
    const SIZE: usize = 1000;
    let mut graph = Graph::new(positions.len());
    distances
        .iter()
        .take(SIZE)
        .for_each(|edge| graph.add_edge(edge.0, edge.1));
    let mut components = graph.components();
    components.sort();
    components.reverse();
    let ans = components[0] * components[1] * components[2];
    println!("{ans}");
}
