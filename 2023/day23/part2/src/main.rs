use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    y: usize,
    x: usize,
}

#[derive(Default)]
struct Graph {
    adj: HashMap<Point, Vec<(Point, usize)>>,
}

impl Graph {
    fn new(grid: &[Vec<char>]) -> Self {
        let mut graph = Self::default();
        const DIRS: [(i32, i32); 2] = [(-1, 0), (0, 1)];
        for (x, row) in grid.iter().enumerate() {
            for (y, &c) in row.iter().enumerate() {
                if c == '#' {
                    continue;
                }
                DIRS.iter()
                    .map(|&(dx, dy)| ((x as i32 + dx) as usize, (y as i32 + dy) as usize))
                    .filter(|&(nx, ny)| grid[nx][ny] != '#')
                    .for_each(|(nx, ny)| graph.add_edge(Point { x, y }, Point { x: nx, y: ny }, 1));
            }
        }
        graph
    }

    fn add_edge(&mut self, p1: Point, p2: Point, weight: usize) {
        self.adj.entry(p1).or_default().push((p2, weight));
        self.adj.entry(p2).or_default().push((p1, weight));
    }

    fn remove_edge(&mut self, p1: Point, p2: Point, weight: usize) {
        if let Some(v) = self.adj.get_mut(&p1) {
            v.retain(|&e| e != (p2, weight));
        }
        if let Some(v) = self.adj.get_mut(&p2) {
            v.retain(|&e| e != (p1, weight));
        }
    }

    fn compress(&mut self) {
        while let Some(node) = self
            .adj
            .iter()
            .find_map(|(p, n)| (n.len() == 2).then_some(*p))
        {
            let [(n1, w1), (n2, w2)] = self.adj.remove(&node).unwrap()[..] else {
                unreachable!()
            };
            self.remove_edge(n1, node, w1);
            self.remove_edge(n2, node, w2);
            self.add_edge(n1, n2, w1 + w2);
        }
    }
    fn dfs(&self, node: &Point, end: &Point, vis: &mut HashSet<Point>) -> Option<usize> {
        if node == end {
            return Some(0);
        }
        let neighbors: Vec<_> = self.adj[node]
            .iter()
            .filter(|(n, _)| !vis.contains(n))
            .collect();
        neighbors
            .iter()
            .filter_map(|&&(n, w)| {
                vis.insert(n);
                let result = self.dfs(&n, end, vis).map(|d| d + w);
                vis.remove(&n);
                result
            })
            .max()
    }
}

fn main() {
    let mut grid: Vec<Vec<char>> = include_str!("../input/input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let width = grid[0].len();
    grid.insert(0, vec!['#'; width]);
    grid.push(vec!['#'; width]);

    let find_open = |row: &[char]| row.iter().position(|&c| c != '#').unwrap();
    let start = Point {
        x: 1,
        y: find_open(&grid[1]),
    };
    let end = Point {
        x: grid.len() - 2,
        y: find_open(&grid[grid.len() - 2]),
    };

    let mut graph = Graph::new(&grid);
    graph.compress();

    let mut vis = HashSet::from([start]);
    println!("{}", graph.dfs(&start, &end, &mut vis).unwrap());
}
