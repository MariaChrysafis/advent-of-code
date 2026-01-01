fn main() {
    let roll_indices = include_str!("../input/input.txt")
        .lines()
        .enumerate().flat_map(
            |(i, str)| {
                str.chars().enumerate().filter(|(j, val)| *val == '@')
                .map(move |x| (i, x.0))
            }
        ).collect::<Vec<(usize, usize)>>();
    let mut ans = 0;
    for &(x1, y1) in roll_indices.iter() {
        let num_roll_neighbors = roll_indices
            .iter()
            .filter(|&&(x2, y2)| {
                (x1 as i32 - x2 as i32).abs() <= 1 && (y1 as i32 - y2 as i32).abs() <= 1
            })
            .count();
        if num_roll_neighbors <= 4 {
            ans += 1;
        }
    }
    println!("{ans}");
}
