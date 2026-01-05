use geo::line_intersection::{LineIntersection, line_intersection};
use geo::{Coord, Intersects, Line, Rect};

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
    let bounds = Rect::new(Coord { x: MIN, y: MIN }, Coord { x: MAX, y: MAX });
    let mut count = 0;
    for (i, segment1) in input.iter().enumerate() {
        for segment2 in input[i + 1..].iter() {
            let in_bounds = match line_intersection(*segment1, *segment2) {
                Some(LineIntersection::SinglePoint { intersection, .. }) => {
                    bounds.intersects(&intersection)
                }
                Some(LineIntersection::Collinear { intersection }) => {
                    bounds.intersects(&intersection)
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
