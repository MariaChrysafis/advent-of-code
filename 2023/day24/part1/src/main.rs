use geo::line_intersection::{LineIntersection, line_intersection};
use geo::{Coord, Line};

fn main() {
    // parse the input
    let input: Vec<Line<f64>> = include_str!("../input/input.txt")
        .lines()
        .map(|line| {
            let (pos, dir) = line.split_once("@").unwrap();
            let pos_parts: Vec<f64> = pos.split(",").map(|s| s.trim().parse().unwrap()).collect();
            let dir_parts: Vec<f64> = dir.split(",").map(|s| s.trim().parse().unwrap()).collect();
            Line {
                start: Coord {
                    x: pos_parts[0],
                    y: pos_parts[1],
                },
                end: Coord {
                    x: pos_parts[0] + dir_parts[0] * 100000000000000000.0,
                    y: pos_parts[1] + dir_parts[1] * 100000000000000000.0,
                },
            }
        })
        .collect();
    const MIN: f64 = 200000000000000.0;
    const MAX: f64 = 400000000000000.0;
    let mut count = 0;
    for (i, segment1) in input.iter().enumerate() {
        for segment2 in input[i + 1..].iter() {
            let in_bounds = match line_intersection(*segment1, *segment2) {
                Some(LineIntersection::SinglePoint { intersection, .. }) => {
                    intersection.x >= MIN
                        && intersection.x <= MAX
                        && intersection.y >= MIN
                        && intersection.y <= MAX
                }
                Some(LineIntersection::Collinear { intersection }) => {
                    // intersection is the overlapping Line segment
                    let x_min = intersection.start.x.min(intersection.end.x);
                    let x_max = intersection.start.x.max(intersection.end.x);
                    let y_min = intersection.start.y.min(intersection.end.y);
                    let y_max = intersection.start.y.max(intersection.end.y);
                    x_max >= MIN && x_min <= MAX && y_max >= MIN && y_min <= MAX
                }
                None => false,
            };
            if in_bounds {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
