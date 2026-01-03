use std::{collections::HashMap, fmt};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Shape {
    coordinates: Vec<(usize, usize)>,
    sz: usize,
}

impl Shape {
    fn _print(&self) {
        for (x, y) in self.coordinates.clone() {
            println!("{} {}", x, y);
        }
        println!();
    }
    fn rotate90(&self) -> Shape {
        Shape {
            coordinates: self
                .coordinates
                .iter()
                .map(|&(x, y)| (self.sz - y - 1, x))
                .collect(),
            sz: self.sz,
        }
    }
    fn flip(&self) -> Shape {
        Shape {
            coordinates: self.coordinates.iter().map(|&(x, y)| (y, x)).collect(),
            sz: self.sz,
        }
    }
}
fn get_all_shapes(shape: Shape) -> Vec<Shape> {
    let mut ans = vec![];
    let mut s = shape;
    for _ in 0..4 {
        s = s.rotate90();
        ans.push(s.clone());
        ans.push(s.flip());
    }
    ans.into_iter()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect()
}
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
    Filled,
    Unfilled,
    Unknown,
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Filled => write!(f, "f"),
            Cell::Unfilled => write!(f, "u"),
            Cell::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Grid {
    grid: Vec<Vec<Cell>>,
}
impl Grid {
    fn set(&self, i: usize, j: usize, cell_type: Cell) -> Grid {
        let mut grid = self.grid.clone();
        grid[i][j] = cell_type;
        Grid { grid }
    }
    fn overlay_shape(&self, shape: Shape, i: usize, j: usize) -> Option<Grid> {
        let mut grid = self.clone();
        for (x, y) in shape.coordinates.iter() {
            let xi = x + i;
            let yj = y + j;
            if xi < grid.grid.len() && yj < grid.grid[0].len() && grid.grid[xi][yj] == Cell::Unknown
            {
                grid.grid[xi][yj] = Cell::Filled;
            } else {
                return None;
            }
        }
        Some(grid)
    }
    fn first(&self, cell_type: Cell) -> Option<(usize, usize)> {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == cell_type {
                    return Some((i, j));
                }
            }
        }
        None
    }
    fn _print(&self) {
        for arr in self.grid.clone() {
            for x in arr {
                print!("{} ", x);
            }
            println!();
        }
    }
}
fn dfs(
    grid: Grid,
    shapes: &Vec<Vec<Shape>>,
    cnts: Vec<i32>,
    dp: &mut HashMap<(Grid, Vec<i32>), bool>,
) -> bool {
    let need_to_fill = (0..cnts.len())
        .map(|i| shapes[i][0].coordinates.len() as i32 * cnts[i])
        .sum::<i32>();
    let can_fill = grid
        .grid
        .iter()
        .flatten()
        .filter(|&x| *x == Cell::Unknown)
        .count() as i32;
    // grid._print();
    println!("need_to_fill: {}", need_to_fill);
    println!("can_fill: {}", can_fill);
    if need_to_fill.abs_diff(can_fill) > 100 {
        return true;
    }
    if need_to_fill > can_fill {
        return false;
    }
    if dp.contains_key(&(grid.clone(), cnts.clone())) {
        return dp[&(grid, cnts)];
    }
    if cnts.iter().sum::<i32>() == 0 {
        return true;
    }

    // find the first position that is unfilled
    let position = grid.first(Cell::Unknown);
    if position.is_none() {
        return false;
    }
    let (x, y) = position.unwrap();
    let mut ans = dfs(grid.set(x, y, Cell::Unfilled), shapes, cnts.clone(), dp);
    if !ans {
        for (i, arr) in shapes.iter().enumerate() {
            if cnts[i] == 0 {
                continue;
            }
            for shape in arr.iter() {
                let new_grid = grid.overlay_shape(shape.clone(), x, y);
                if let Some(item) = new_grid {
                    let mut new_cnts = cnts.clone();
                    new_cnts[i] -= 1;
                    ans |= dfs(item, shapes, new_cnts, dp);
                    if ans {
                        break;
                    }
                }
            }
        }
    }
    dp.insert((grid, cnts), ans);
    ans
}
fn solve(shapes: &Vec<Vec<Shape>>, dimensions: (usize, usize), cnts: Vec<i32>) -> bool {
    dfs(
        Grid {
            grid: vec![vec![Cell::Unknown; dimensions.0]; dimensions.1],
        },
        shapes,
        cnts,
        &mut HashMap::new(),
    )
}
fn main() {
    let points: Vec<&str> = include_str!("../input/input.txt").split("\n\n").collect();
    let trees: Vec<Vec<Shape>> = points[..points.len() - 1]
        .to_vec()
        .iter()
        .map(|x| {
            let tree: Vec<&str> = x.split("\n").collect::<Vec<&str>>()[1..].to_vec();
            let mut coordinates = vec![];
            for (i, str) in tree.iter().enumerate() {
                for (j, c) in str.chars().enumerate() {
                    if c == '#' {
                        coordinates.push((i, j));
                    }
                }
            }
            get_all_shapes(Shape { coordinates, sz: 3 })
        })
        .collect();
    let regions: Vec<((usize, usize), Vec<i32>)> = points[points.len() - 1]
        .split("\n")
        .map(|s| {
            let (dimensions, counts) = s.split_once(":").unwrap();
            (
                {
                    let (w, h) = dimensions
                        .split_once("x")
                        .map(|x| (x.0.parse::<usize>().unwrap(), x.1.parse::<usize>().unwrap()))
                        .unwrap();
                    (w, h)
                },
                counts
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
            )
        })
        .collect();
    let mut sum = 0;
    for region in regions {
        let ans = solve(&trees, region.0, region.1);
        println!("{}", ans);
        sum += ans as i32;
        // return;
    }
    println!("Sum: {}", sum);
}
