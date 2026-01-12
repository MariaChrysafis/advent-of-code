type Point = [i32; 2];
fn solve(grid: &Vec<Vec<char>>, point: Point, d: i32) -> Vec<Point> {
    if d == 1 {
        return [[-1, 0], [0, 1], [0, -1], [1, 0]]
            .iter()
            .map(|[dx, dy]| [dx + point[0], dy + point[1]] as Point)
            .filter(|point| {
                point[0] >= 0
                    && point[1] >= 0
                    && point[0] < grid.len() as i32
                    && point[1] < grid[0].len() as i32
                    && grid[point[0] as usize][point[1] as usize] != '#'
            })
            .collect();
    }
    let mut ans = Vec::new();
    for point in solve(grid, point, d - 1) {
        ans.extend(solve(grid, point, 1));
    }
    ans.sort();
    ans.dedup();
    ans
}
fn main() {
    let grid: Vec<Vec<char>> = include_str!("../../input/input.txt")
        .split("\n")
        .map(|str| str.chars().collect())
        .collect();
    let mut point: Point = [0, 0];
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                point = [i as i32, j as i32];
            }
        }
    }
    let ans = solve(&grid, point, 64);
    println!("{}", ans.len());
}
