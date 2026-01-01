fn num_removable_rolls(roll_indices: Vec<(usize, usize)>) -> i32 {
    let to_remove: Vec<&(usize, usize)> = roll_indices
        .iter()
        .filter(|(x1, y1)| {
            let num_roll_neighbors = roll_indices
                .iter()
                .filter(|&&(x2, y2)| {
                    (*x1 as i32 - x2 as i32).abs() <= 1 && (*y1 as i32 - y2 as i32).abs() <= 1
                })
                .count();
            return num_roll_neighbors <= 4;
        })
        .collect();
    if to_remove.len() == 0 {
        return 0;
    }
    let mut next_input = roll_indices.clone();
    next_input.retain(|x| !to_remove.contains(&x));
    return num_removable_rolls(next_input) + to_remove.len() as i32;
}
fn main() {
    let roll_indices = include_str!("../input/input.txt")
        .lines()
        .enumerate()
        .flat_map(|(i, str)| {
            str.chars()
                .enumerate()
                .filter(|(j, val)| *val == '@')
                .map(move |x| (i, x.0))
        })
        .collect::<Vec<(usize, usize)>>();
    let ans = num_removable_rolls(roll_indices);
    println!("{ans}");
}
