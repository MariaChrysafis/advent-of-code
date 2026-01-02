fn main() {
    let positions: Vec<(i64, i64)> = include_str!("../input/input.txt")
        .lines()
        .map(|x| x.split_once(",").unwrap())
        .map(|x| (x.0.parse::<i64>().unwrap(), x.1.parse::<i64>().unwrap()))
        .collect();
    let ans: i64 = positions
        .iter()
        .enumerate()
        .flat_map(|(i, coord1)| {
            positions[i + 1..].iter().map(move |coord2| {
                ((coord1.0 - coord2.0).abs() + 1) * ((coord2.1 - coord1.1).abs() + 1)
            })
        })
        .max()
        .unwrap();
    print!("{ans}");
}
