use std::cmp;
#[derive(PartialEq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}
fn valid_pairs(input: &[Point]) -> Vec<(Point, Point)> {
    let sz: usize = input.iter().map(|p| cmp::max(p.x, p.y) + 1).max().unwrap() as usize;
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
                            line.0.x == line.1.x
                                && i < line.0.x as usize
                                && j > cmp::min(line.0.y, line.1.y) as usize
                                && j <= cmp::max(line.0.y, line.1.y) as usize
                        })
                        .count()
                        % 2
                        == 1
                })
                .collect::<Vec<bool>>()
        })
        .collect();
    for line in lines {
        for i in cmp::min(line.0.x, line.1.x)..=cmp::max(line.0.x, line.1.x) {
            is_inside[i as usize][line.1.y as usize] = true;
        }
    }
    input
        .iter()
        .flat_map(|&point1| {
            input
                .iter()
                .map(move |&point2| (point1, point2))
                .filter(|&(point1, point2)| {
                    (point1.x.min(point2.x)..=point1.x.max(point2.x))
                        .flat_map(|i| {
                            (point1.y.min(point2.y)..=point1.y.max(point2.y)).map(move |j| (i, j))
                        })
                        .all(|(i, j)| is_inside[i as usize][j as usize])
                })
        })
        .collect()
}
fn main() {
    // tests();
    let points: Vec<Point> = include_str!("../input/input.txt")
        .lines()
        .map(|x| x.split_once(",").unwrap())
        .map(|x| (x.0.parse::<i64>().unwrap(), x.1.parse::<i64>().unwrap()))
        .map(|(x, y)| Point { x, y })
        .collect();
    // coordinate compression (map a large coordinate to a smaller one)
    let mut positions: Vec<i64> = points
        .iter()
        .flat_map(|point| vec![point.x, point.y])
        .collect();
    positions.sort();
    positions.dedup();
    let compressed_points: Vec<Point> = points
        .iter()
        .map(|point| Point {
            x: positions.iter().position(|&val| val == point.x).unwrap() as i64 * 2,
            y: positions.iter().position(|&val| val == point.y).unwrap() as i64 * 2,
        })
        .collect();
    let result = valid_pairs(&compressed_points);
    let mut ans = 0;
    for (point1, point2) in result {
        let dx = positions[(point1.x as usize) / 2].abs_diff(positions[(point2.x as usize) / 2]);
        let dy = positions[(point1.y as usize) / 2].abs_diff(positions[(point2.y as usize) / 2]);
        ans = cmp::max(ans, (dx + 1) * (dy + 1));
    }
    println!("{}", ans);
}
