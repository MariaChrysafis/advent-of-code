fn main() {
    let grid: Vec<Vec<char>> = include_str!("../input/input.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    let indices = grid.iter().enumerate().flat_map(
        |(i, _)| {
            grid[i].iter().enumerate().filter(|(j, val)| *(*val) == '@').map(move |x| (i, x.0))
        }
    ).collect::<Vec<(usize, usize)>>();
    let mut ans = 0;
    for &(x1, y1) in indices.iter() {
        let mut cnt = 0;
        for &(x2, y2) in indices.iter() {
            let dx = (x1 as i32 - x2 as i32).abs();
            let dy = (y1 as i32 - y2 as i32).abs();
            if -1 <= dx && dx <= 1 && -1 <= dy && dy <= 1 {
                cnt += 1
            }
        }
        if cnt <= 4 {
            ans += 1;
        }
    }
    println!("{ans}");
}
