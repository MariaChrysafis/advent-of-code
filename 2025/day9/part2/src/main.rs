type Point = [i64; 2];

fn valid_pairs(input: &[Point]) -> Vec<(Point, Point)> {
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
                .collect::<Vec<bool>>()
        })
        .collect();
    for line in lines {
        for i in line.0[0].min(line.1[0])..=line.0[0].max(line.1[0]) {
            is_inside[i as usize][line.1[1] as usize] = true;
        }
    }
    input
        .iter()
        .flat_map(|&point1| {
            input
                .iter()
                .map(move |&point2| (point1, point2))
                .filter(|&(point1, point2)| {
                    (point1[0].min(point2[0])..=point1[0].max(point2[0]))
                        .flat_map(|i| {
                            (point1[1].min(point2[1])..=point1[1].max(point2[1]))
                                .map(move |j| (i, j))
                        })
                        .all(|(i, j)| is_inside[i as usize][j as usize])
                })
        })
        .collect()
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
    let ans = valid_pairs(&compressed_points)
        .into_iter()
        .map(|(p1, p2)| {
            (0..2)
                .map(|i| positions[p1[i] as usize].abs_diff(positions[p2[i] as usize]) + 1)
                .product::<u64>()
        })
        .max()
        .unwrap();
    println!("{}", ans);
}
