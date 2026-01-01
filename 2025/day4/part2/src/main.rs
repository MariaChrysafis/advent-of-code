fn num_removable_rolls(roll_indices: Vec<(usize, usize)>) -> i32 {
    let mut to_remove = vec![];
    for &(x1, y1) in roll_indices.iter() {
        let mut cnt = 0;
        for &(x2, y2) in roll_indices.iter() {
            if (x1 as i32 - x2 as i32).abs() <= 1 && (y1 as i32 - y2 as i32).abs() <= 1 {
                cnt += 1;
            }
        }
        if cnt <= 4 {
            to_remove.push((x1, y1));
        }
    }
    if to_remove.len() == 0 {
        return 0;
    }
    let mut next_input = roll_indices;
    next_input.retain(|x| !to_remove.contains(&x));
    return num_removable_rolls(next_input) + to_remove.len() as i32;
}
fn main() {
    let roll_indices = include_str!("../input/sample.txt")
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
