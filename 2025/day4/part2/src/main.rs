fn num_removable_rolls(mut roll_indices: Vec<(usize, usize)>) -> i32 {
    let to_remove: Vec<(usize, usize)> = roll_indices
        .iter()
        .copied()
        .filter(|&(x1, y1)| {
            let num_roll_neighbors = roll_indices
                .iter()
                .filter(|&&(x2, y2)| {
                    (x1 as i32 - x2 as i32).abs() <= 1 && (y1 as i32 - y2 as i32).abs() <= 1
                })
                .count();
            num_roll_neighbors <= 4
        })
        .collect();
    if to_remove.is_empty() {
        return 0;
    }
    roll_indices.retain(|x| !to_remove.contains(x));
    num_removable_rolls(roll_indices) + to_remove.len() as i32
}
fn main() {
    let roll_indices = include_str!("../../input/input.txt")
        .lines()
        .enumerate()
        .flat_map(|(i, str)| {
            str.chars()
                .enumerate()
                .filter(|(_, val)| *val == '@')
                .map(move |(j, _): (usize, char)| (i, j))
        })
        .collect::<Vec<(usize, usize)>>();
    let ans = num_removable_rolls(roll_indices);
    println!("{ans}");
}
