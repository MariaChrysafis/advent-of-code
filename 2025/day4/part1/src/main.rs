fn main() {
    let grid: Vec<Vec<char>> = include_str!("../input/input.txt")
        .lines()
        .map(|x| x.chars().collect()).collect();
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '.' {
                continue;
            }
            let mut cnt = 0;
            for dx in -1i32..=1 {
                for dy in -1i32..=1 {
                    if !(dx == 0 && dy == 0) {
                        let new_x = i as i32 + dx;
                        let new_y = j as i32 + dy;
                        if new_x < 0
                            || new_y < 0
                            || new_x >= grid.len() as i32
                            || new_y >= grid[i].len() as i32
                        {
                            continue;
                        }
                        if grid[new_x as usize][new_y as usize] == '@' {
                            cnt += 1;
                        }
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
