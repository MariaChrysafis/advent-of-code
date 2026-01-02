struct Graph {
    parent: Vec<usize>,
    sz: Vec<usize>,
}
impl Graph {
    pub fn find_head(&self, u: usize) -> usize {
        if u == self.parent[u] {
            u
        } else {
            self.find_head(self.parent[u])
        }
    }
    pub fn join(&mut self, mut u: usize, mut v: usize) {
        u = self.find_head(u);
        v = self.find_head(v);
        if u != v {
            self.parent[u] = v;
            self.sz[v] += self.sz[u];
        }
    }
    pub fn new(n: usize) -> Graph {
        Graph {
            parent: (0..n).collect(),
            sz: vec![1; n],
        }
    }
}
fn main() {
    let positions: Vec<Vec<i64>> = include_str!("../input/input.txt")
        .lines()
        .map(|x| x.split(',').map(|x| x.parse::<i64>().unwrap()).collect())
        .collect();
    let mut edges: Vec<(usize, usize)> = (0..positions.len())
        .flat_map(|i| (i + 1..positions.len()).map(move |j| (i, j)))
        .collect();
    edges.sort_by_key(|&(i, j)| {
        positions[i]
            .iter()
            .zip(&positions[j])
            .map(|(a, b)| (a - b).pow(2))
            .sum::<i64>()
    });
    let mut graph = Graph::new(positions.len());
    for (i, j) in edges {
        graph.join(i, j);
        if graph.sz[graph.find_head(i)] == positions.len() {
            let ans = positions[i][0] * positions[j][0];
            print!("{ans}");
            return;
        }
    }
}
