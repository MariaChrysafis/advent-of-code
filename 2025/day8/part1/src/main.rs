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
fn distance(pos1: (i64, i64, i64), pos2: (i64, i64, i64)) -> i64 {
    (pos1.0 - pos2.0).pow(2) + (pos1.1 - pos2.1).pow(2) + (pos1.2 - pos2.2).pow(2)
}
fn main() {
    let positions = include_str!("../input/sample.txt")
        .lines()
        .map(|x| {
            x.split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|x| (x[0], x[1], x[2]))
        .collect::<Vec<(i64, i64, i64)>>();
    let mut distances: Vec<(i64, (usize, usize))> = vec![];
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            distances.push((distance(positions[i], positions[j]), (i, j)));
        }
    }
    distances.sort();
    const SIZE: usize = 1000;
    let mut graph = Graph::new(positions.len());
    for edge in distances.iter().take(SIZE) {
        graph.add_edge(edge.1.0, edge.1.1);
    }
    let mut components = graph.components();
    components.sort();
    components.reverse();
    let ans = components[0] * components[1] * components[2];
    println!("{ans}");
}
