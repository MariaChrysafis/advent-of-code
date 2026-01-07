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
        return Some(self.grid[point.x as usize][point.y as usize]);
    }
    fn all_points(&self) -> Vec<Point> {
        return (0..self.grid.len())
            .flat_map(|x| {
                (0..self.grid[0].len())
                    .map(|y| Point {
                        x: x as i32,
                        y: y as i32,
                    })
                    .collect::<Vec<Point>>()
            })
            .collect();
    }
    fn neighbors(&self, point: Point) -> Vec<Point> {
        match self.get(point).unwrap() {
            '>' => vec![Point {
                x: point.x,
                y: point.y + 1,
            }],
            '<' => vec![Point {
                x: point.x,
                y: point.y - 1,
            }],
            'v' => vec![Point {
                x: point.x + 1,
                y: point.y,
            }],
            '^' => vec![Point {
                x: point.x - 1,
                y: point.y,
            }],
            '.' => [[-1, 0], [1, 0], [0, 1], [0, -1]]
                .map(|delta| Point {
                    x: point.x + delta[0],
                    y: point.y + delta[1],
                })
                .into_iter()
                .collect(),
            _ => panic!(),
        }
    }
}
fn dfs(grid: &Grid, coord: Point, past_points: &Vec<Point>) -> i32 {
    if grid.get(coord) == Some('#') || grid.get(coord) == None {
        return 0;
    }
    let mut ans = 0;
    for neighbor in grid
        .neighbors(coord)
        .iter()
        .filter(|point| !past_points.contains(point))
    {
        let mut new_past_points = past_points.clone();
        new_past_points.push(*neighbor);
        ans = ans.max(dfs(grid, *neighbor, &new_past_points));
    }
    return ans + 1;
}
fn main() {
    let g: Vec<Vec<char>> = include_str!("../input/sample.txt")
        .split("\n")
        .map(|x| x.chars().into_iter().collect())
        .collect::<Vec<Vec<char>>>();
    let grid = Grid { grid: g };
    let ans = grid
        .all_points()
        .iter()
        .map(|&point| dfs(&grid, point, &vec![point]))
        .max()
        .unwrap()
        - 1;
    println!("{ans}");
}
