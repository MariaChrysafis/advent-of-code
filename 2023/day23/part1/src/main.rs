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
    fn start(&self) -> Point {
        Point {
            x: 0,
            y: self.grid[0].iter().position(|c| *c == '.').unwrap() as i32,
        }
    }
    fn neighbors(&self, point: Point) -> Vec<Point> {
        let deltas: &[[i32; 2]] = match self.get(point).unwrap() {
            '>' => &[[0, 1]],
            '<' => &[[0, -1]],
            'v' => &[[1, 0]],
            '^' => &[[-1, 0]],
            '.' => &[[0, 1], [0, -1], [1, 0], [-1, 0]],
            _ => panic!(),
        };

        deltas
            .iter()
            .map(|[dx, dy]| Point {
                x: point.x + dx,
                y: point.y + dy,
            })
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
    let g: Vec<Vec<char>> = include_str!("../input/input.txt")
        .split("\n")
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let grid = Grid { grid: g };
    let ans = dfs(&grid, grid.start(), vec![grid.start()]) - 1;
    println!("{ans}");
}
