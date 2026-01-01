fn main() {
    let grid: Vec<Vec<char>> = include_str!("../input/input.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    let roll_indices = grid
        .iter()
        .enumerate()
        .flat_map(|(i, _)| {
            grid[i]
                .iter()
                .enumerate()
                .filter(|(j, val)| *(*val) == '@')
                .map(move |x| (i, x.0))
        })
        .collect::<Vec<(usize, usize)>>();
    let mut ans = 0;
    for &(x1, y1) in roll_indices.iter() {
        let cnt = roll_indices
            .iter()
            .filter(|&&(x2, y2)| {
                (x1 as i32 - x2 as i32).abs() <= 1 && (y1 as i32 - y2 as i32).abs() <= 1
            })
            .count();
        if cnt <= 4 {
            ans += 1;
        }
    }
    println!("{ans}");
}
