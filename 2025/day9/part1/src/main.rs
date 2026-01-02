fn main() {
    let positions: Vec<(i64, i64)> = include_str!("../input/input.txt")
        .lines()
        .map(|x| x.split_once(",").unwrap())
        .map(|x| (x.0.parse::<i64>().unwrap(), x.1.parse::<i64>().unwrap()))
        .collect();
    let positions = &positions;
    let ans: i64 = (0..positions.len())
        .flat_map(|i| {
            ((i + 1)..positions.len()).map(move |j| {
                ((positions[i].0 - positions[j].0).abs() + 1)
                    * ((positions[i].1 - positions[j].1).abs() + 1)
            })
        })
        .max()
        .unwrap();
    print!("{ans}");
}
