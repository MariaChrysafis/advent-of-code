type Point = [i64; 2];

fn max_valid_area(input: &[Point], positions: &[i64]) -> u64 {
    let sz: usize = input.iter().map(|p| p[0].max(p[1]) + 1).max().unwrap() as usize;
    let lines: Vec<(Point, Point)> = (0..input.len())
        .map(|i| (input[i], input[(i + 1) % input.len()]))
        .collect();
    let mut is_inside: Vec<Vec<bool>> = (0..sz)
        .map(|i| {
            (0..sz)
                .map(|j| {
                    lines
                        .iter()
                        .filter(|line| {
                            line.0[0] == line.1[0]
                                && i < line.0[0] as usize
                                && j > line.0[1].min(line.1[1]) as usize
                                && j <= line.0[1].max(line.1[1]) as usize
                        })
                        .count()
                        % 2
                        == 1
                })
                .collect()
        })
        .collect();
    for line in lines {
        for i in line.0[0].min(line.1[0])..=line.0[0].max(line.1[0]) {
            is_inside[i as usize][line.1[1] as usize] = true;
        }
    }
    let is_valid = |p1: Point, p2: Point| {
        (p1[0].min(p2[0])..=p1[0].max(p2[0]))
            .flat_map(|i| (p1[1].min(p2[1])..=p1[1].max(p2[1])).map(move |j| (i, j)))
            .all(|(i, j)| is_inside[i as usize][j as usize])
    };
    let area = |p1: Point, p2: Point| {
        (0..2)
            .map(|i| positions[p1[i] as usize].abs_diff(positions[p2[i] as usize]) + 1)
            .product()
    };
    input
        .iter()
        .flat_map(|&p1| input.iter().map(move |&p2| (p1, p2)))
        .filter(|&(p1, p2)| is_valid(p1, p2))
        .map(|(p1, p2)| area(p1, p2))
        .max()
        .unwrap()
}
fn main() {
    let points: Vec<Point> = include_str!("../input/input.txt")
        .lines()
        .map(|x| {
            let (a, b) = x.split_once(",").unwrap();
            [a.parse().unwrap(), b.parse().unwrap()]
        })
        .collect();
    let mut positions: Vec<i64> = points.iter().flat_map(|point| *point).collect();
    positions.sort();
    positions.dedup();
    let compressed_points: Vec<Point> = points
        .iter()
        .map(|point| point.map(|c| positions.binary_search(&c).unwrap() as i64))
        .collect();
    println!("{}", max_valid_area(&compressed_points, &positions));
}
