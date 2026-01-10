use std::cmp;

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    y: i32,
    x: i32,
}
struct State {
    grid: Vec<Vec<char>>,
    coord: Point,
    has_visited: Vec<Vec<bool>>,
}
impl State {
    fn from(grid: Vec<Vec<char>>) -> State {
        let coord = Point {
            x: 0,
            y: grid[0].iter().position(|&c| c != '#').unwrap() as i32,
        };
        let mut has_visited = vec![vec![false; grid.len()]; grid[0].len()];
        has_visited[coord.x as usize][coord.y as usize] = true;
        State {
            grid: grid.clone(),
            coord,
            has_visited,
        }
    }
    fn has_visited(&self, point: &Point) -> bool {
        self.has_visited[point.x as usize][point.y as usize]
    }
    fn get(&self, point: &Point) -> Option<char> {
        if point.x <= 0
            || point.y < 0
            || point.x >= self.grid.len() as i32
            || point.y >= self.grid[0].len() as i32
        {
            return None;
        }
        Some(self.grid[point.x as usize][point.y as usize])
    }
    fn neighbors(&self, point: &Point) -> Vec<Point> {
        [[-1, 0], [1, 0], [0, -1], [0, 1]]
            .iter()
            .map(|[dx, dy]| Point {
                x: point.x + dx,
                y: point.y + dy,
            })
            .filter(|point| self.get(point).is_some_and(|x| x != '#') && !self.has_visited(point))
            .collect()
    }
}

//longest distance till we reach the bottom
fn dfs(state: &mut State) -> i32 {
    assert!(state.has_visited[state.coord.x as usize][state.coord.y as usize]);
    let mut d = 1;
    while state.neighbors(&state.coord).len() == 1 {
        let next_point = state.neighbors(&state.coord)[0];
        state.has_visited[next_point.x as usize][next_point.y as usize] = true;
        state.coord = next_point;
        d += 1;
    }
    if state.coord.x == state.grid.len() as i32 - 1 {
        return d;
    }
    let mut ans = -100000000;
    let current_point = state.coord;
    for neighbor in state.neighbors(&state.coord) {
        state.has_visited[neighbor.x as usize][neighbor.y as usize] = true;
        state.coord = neighbor;
        ans = cmp::max(ans, dfs(state) + d);
        state.has_visited[neighbor.x as usize][neighbor.y as usize] = false;
        state.coord = current_point;
    }
    ans
}

fn main() {
    let g: Vec<Vec<char>> = include_str!("../input/input.txt")
        .split("\n")
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let ans = dfs(&mut State::from(g));
    println!("{}", ans - 1);
}
