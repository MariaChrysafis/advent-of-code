const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    let grid: Vec<Vec<char>> = include_str!("../input/input.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '.' {
                continue;
            }
            let mut cnt = 0;
            for (dx, dy) in DIRS {
                if !(dx == 0 && dy == 0) {
                    let new_x = i as isize + dx;
                    let new_y = j as isize + dy;
                    if new_x < 0
                        || new_y < 0
                        || new_x >= grid.len() as isize
                        || new_y >= grid[i].len() as isize
                    {
                        continue;
                    }
                    if grid[new_x as usize][new_y as usize] == '@' {
                        cnt += 1;
                    }
                }
            }
            if cnt < 4 {
                ans += 1;
            }
        }
    }
    println!("{ans}");
}
