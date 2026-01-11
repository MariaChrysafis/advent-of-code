use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    y: usize,
    x: usize,
}

struct Graph {
    adj: HashMap<Point, Vec<(Point, usize)>>,
}

trait HashMapVecExt<T, U> {
    fn remove_from_vec(&mut self, key: T, val: U);
    fn add_to_vec(&mut self, key: T, val: U);
}

impl<T, U> HashMapVecExt<T, U> for HashMap<T, Vec<U>>
where
    T: std::hash::Hash + Eq,
    U: PartialEq + Copy,
{
    fn remove_from_vec(&mut self, key: T, val: U) {
        if let Some(vec) = self.get_mut(&key) {
            vec.retain(|&x| x != val);
        }
    }

    fn add_to_vec(&mut self, key: T, val: U) {
        self.entry(key).or_default().push(val);
    }
}

impl Graph {
    fn from(grid: Vec<Vec<char>>) -> Graph {
        let mut ans = Graph {
            adj: HashMap::new(),
        };
        const DIRECTIONS: [[i32; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];
        for (x, row) in grid.iter().enumerate() {
            for (y, c) in row.iter().enumerate() {
                if *c == '#' {
                    continue;
                }
                for [dx, dy] in DIRECTIONS {
                    let new_x = (x as i32 + dx) as usize;
                    let new_y = (y as i32 + dy) as usize;
                    if grid[new_x][new_y] != '#' {
                        let p1 = Point { x, y };
                        let p2 = Point { x: new_x, y: new_y };
                        if p1 < p2 {
                            ans.add_edge(p1, p2, 1);
                        }
                    }
                }
            }
        }
        ans
    }
    fn add_edge(&mut self, p1: Point, p2: Point, weight: usize) {
        self.adj.add_to_vec(p1, (p2, weight));
        self.adj.add_to_vec(p2, (p1, weight));
    }

    fn remove_edge(&mut self, p1: Point, p2: Point, weight: usize) {
        self.adj.remove_from_vec(p1, (p2, weight));
        self.adj.remove_from_vec(p2, (p1, weight));
    }

    fn find_degree_2_node(&self) -> Option<Point> {
        self.adj
            .iter()
            .find(|(_, neighbors)| neighbors.len() == 2)
            .map(|(p, _)| *p)
    }

    fn compress(&mut self) {
        while let Some(node) = self.find_degree_2_node() {
            let neighbors: Vec<(Point, usize)> = self.adj.remove(&node).unwrap();
            let (node1, weight1) = neighbors[0];
            let (node2, weight2) = neighbors[1];
            self.remove_edge(node1, node, weight1);
            self.remove_edge(node2, node, weight2);
            self.add_edge(node1, node2, weight1 + weight2);
        }
    }
    fn dfs(&self, node: &Point, end: &Point, vis: &mut HashSet<Point>) -> Option<usize> {
        if node == end {
            return Some(0);
        }
        let mut ans: Option<usize> = None;
        for (neighbor, weight) in &self.adj[node] {
            if vis.contains(neighbor) {
                continue;
            }
            vis.insert(*neighbor);
            if let Some(sub) = self.dfs(neighbor, end, vis) {
                ans = Some(ans.unwrap_or(0).max(weight + sub));
            }
            vis.remove(neighbor);
        }
        ans
    }
}

fn main() {
    let mut g: Vec<Vec<char>> = include_str!("../input/input.txt")
        .split("\n")
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    g.insert(0, vec!['#'; g[0].len()]);
    g.push(vec!['#'; g[0].len()]);
    let start = Point {
        x: 1,
        y: g[1].iter().position(|&c| c != '#').unwrap(),
    };
    let end = Point {
        x: g.len() - 2,
        y: g[g.len() - 2].iter().position(|&c| c != '#').unwrap(),
    };
    let mut graph = Graph::from(g);
    graph.compress();
    let mut vis = HashSet::new();
    vis.insert(start);
    let ans = graph.dfs(&start, &end, &mut vis);
    println!("{:?}", ans.unwrap());
}
