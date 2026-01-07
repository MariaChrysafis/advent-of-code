#[derive(Copy, Clone, PartialEq, Eq)]
struct Point {
    y: i32,
    x: i32,
}
struct Grid {
    grid: Vec<Vec<char>>,
}
impl Grid {
    fn get(&self, point: Point) -> Option<char> {
        if point.x < 0
            || point.y < 0
            || point.x >= self.grid.len() as i32
            || point.y >= self.grid[0].len() as i32
        {
            return None;
        }
        Some(self.grid[point.x as usize][point.y as usize])
    }
    fn all_points(&self) -> Vec<Point> {
        (0..self.grid.len())
            .flat_map(|x| {
                (0..self.grid[0].len())
                    .map(|y| Point {
                        x: x as i32,
                        y: y as i32,
                    })
                    .collect::<Vec<Point>>()
            })
            .collect()
    }
    fn neighbors(&self, point: Point) -> Vec<Point> {
        match self.get(point).unwrap() {
            '>' => vec![[0, 1]],
            '<' => vec![[0, -1]],
            'v' => vec![[1, 0]],
            '^' => vec![[1, -1]],
            '.' => vec![[0, 1], [0, -1], [1, 0], [-1, 0]],
            _ => panic!(),
        }
        .iter()
        .map(|delta| Point {
            x: point.x + delta[0],
            y: point.y + delta[1],
        })
        .into_iter()
        .collect()
    }
}
fn dfs(grid: &Grid, coord: Point, past_points: Vec<Point>) -> i32 {
    if grid.get(coord) == Some('#') || grid.get(coord).is_none() {
        return 0;
    }
    grid.neighbors(coord)
        .iter()
        .filter(|point| !past_points.contains(point))
        .map(|&neighbor| {
            let mut new_past_points = past_points.clone();
            new_past_points.push(neighbor);
            dfs(grid, neighbor, new_past_points)
        })
        .max()
        .unwrap_or_default()
        + 1
}
fn main() {
    let g: Vec<Vec<char>> = include_str!("../input/sample.txt")
        .split("\n")
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let grid = Grid { grid: g };
    let ans = grid
        .all_points()
        .iter()
        .map(|&point| dfs(&grid, point, vec![point]))
        .max()
        .unwrap()
        - 1;
    println!("{ans}");
}
