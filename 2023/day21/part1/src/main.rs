type Point = [usize; 2];
fn solve(grid: &Vec<Vec<char>>, point: Point, d: i32) -> Vec<Point> {
    if d == 1 {
        return [[-1, 0], [0, 1], [0, -1], [1, 0]]
            .iter()
            .filter_map(|[dx, dy]| {
                let x = point[0].checked_add_signed(*dx)?;
                let y = point[1].checked_add_signed(*dy)?;
                if x < grid.len() && y < grid[0].len() && grid[x][y] != '#' {
                    Some([x, y])
                } else {
                    None
                }
            })
            .collect();
    }
    let mut ans: Vec<Point> = solve(grid, point, d - 1)
        .into_iter()
        .flat_map(|p| solve(grid, p, 1))
        .collect();
    ans.sort();
    ans.dedup();
    ans
}
fn main() {
    let grid: Vec<Vec<char>> = include_str!("../../input/input.txt")
        .split("\n")
        .map(|str| str.chars().collect())
        .collect();
    let point: Point = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &c)| (i, j, c)))
        .find(|&(_, _, c)| c == 'S')
        .map(|(i, j, _)| [i, j])
        .unwrap();
    let ans = solve(&grid, point, 64);
    println!("{}", ans.len());
}
