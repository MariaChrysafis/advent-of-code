use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    y: usize,
    x: usize,
}

struct Graph {
    adj: HashMap<Point, Vec<(Point, usize)>>,
    deg: HashMap<usize, Vec<Point>>, // maps degrees to list of nodes with that degree
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
            deg: HashMap::new(),
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
                            ans.modify_edge(&p1, &p2, 1, true);
                        }
                    }
                }
            }
        }
        ans
    }
    fn degree(&self, point: &Point) -> usize {
        self.adj.get(point).map(|v| v.len()).unwrap_or(0)
    }
    fn modify_edge(&mut self, point1: &Point, point2: &Point, weight: usize, to_add: bool) {
        for point in [point1, point2] {
            let d = self.degree(point);
            self.deg.remove_from_vec(d, *point);
        }
        if to_add {
            self.adj.add_to_vec(*point1, (*point2, weight));
            self.adj.add_to_vec(*point2, (*point1, weight));
        } else {
            self.adj.remove_from_vec(*point1, (*point2, weight));
            self.adj.remove_from_vec(*point2, (*point1, weight));
        }
        for point in [point1, point2] {
            let d = self.degree(point);
            self.deg.add_to_vec(d, *point);
        }
    }

    fn compress(&mut self) {
        // repeatedly find edges of degree 2 and remove them in favor of a single edge
        while let Some(&node) = self.deg.get(&2).and_then(|v| v.first()) {
            let neighbors: Vec<(Point, usize)> = self
                .adj
                .get(&node)
                .unwrap()
                .clone()
                .into_iter()
                .take(2)
                .collect();
            let weight1 = neighbors[0].1;
            let weight2 = neighbors[1].1;
            let node1 = neighbors[0].0;
            let node2 = neighbors[1].0;
            self.modify_edge(&node1, &node2, weight1 + weight2, true);
            self.modify_edge(&node1, &node, weight1, false);
            self.modify_edge(&node2, &node, weight2, false);
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
