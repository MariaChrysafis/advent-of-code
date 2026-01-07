use core::panic;

#[derive(Copy, Clone, PartialEq, Eq)]
struct Point {
    y: i32,
    x: i32,
}
struct Grid {
    grid: Vec<Vec<char>>,
}
impl Grid {
    fn get(&self, point: Point) -> char {
        return self.grid[point.x as usize][point.y as usize];
    }
    fn start(&self) -> Point {
        return Point { x: 0, y: 0 };
    }
    fn end(&self) -> Point {
        return Point {
            x: (self.grid.len() - 1) as i32,
            y: (self.grid[0].len() - 1) as i32,
        };
    }
    fn neighbors(&self, point: Point) -> Vec<Point> {
        let ans = match self.get(point) {
            '>' => vec![Point {
                x: point.x,
                y: point.y + 1,
            }],
            '<' => vec![Point {
                x: point.x,
                y: point.y - 1,
            }],
            'v' => vec![Point {
                x: point.x,
                y: point.y + 1,
            }],
            '^' => vec![Point {
                x: point.x,
                y: point.y - 1,
            }],
            '.' => (-1..=1)
                .flat_map(|dx| {
                    (-1..=1)
                        .map(|dy| Point {
                            x: point.x + dx,
                            y: point.y + dy,
                        })
                        .collect::<Vec<Point>>()
                })
                .collect(),
            _ => panic!(),
        };
        ans.into_iter()
            .filter(|&point| self.get(point) != '#')
            .collect()
    }
}
fn dfs(grid: &Grid, coord: Point, past_points: &Vec<Point>) -> i32 {
    let mut ans = 0;
    if grid.end() == coord {
        ans += 1;
    }
    for neighbor in grid.neighbors(coord) {
        if past_points.contains(&neighbor) {
            continue;
        }
        let mut new_past_points = past_points.clone();
        new_past_points.push(neighbor);
        ans += dfs(grid, neighbor, past_points);
    }
    return ans;
}
fn main() {
    let g: Vec<Vec<char>> = include_str!("../input/sample.txt")
        .split("\n")
        .map(|x| x.chars().into_iter().collect())
        .collect::<Vec<Vec<char>>>();
    let grid = Grid { grid: g };
    let ans = dfs(&grid, grid.start(), &vec![]);
    println!("{ans}");
}
