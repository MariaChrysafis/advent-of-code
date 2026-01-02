fn main() {
    let mut input = include_str!("../input/input.txt")
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>());
    let mut beams: Vec<i32> = vec![
        input
            .next()
            .unwrap()
            .iter()
            .position(|x| *x == 'S')
            .unwrap() as i32,
    ];
    let mut ans = 0;
    for row in input {
        let mut new_beams: Vec<i32> = beams
            .iter()
            .map(|j| {
                if row[*j as usize] != '^' {
                    return vec![*j];
                } else {
                    ans += 1;
                    return vec![j - 1, j + 1];
                }
            })
            .collect::<Vec<Vec<i32>>>()
            .concat()
            .iter()
            .filter(|&x| *x >= 0 && *x < row.len() as i32)
            .copied()
            .collect();
        new_beams.dedup();
        beams = new_beams;
    }
    // let ans = beams.len();
    println!("{ans}")
}
