use geo::{Intersects, Rect, coord};

type Segment = [i32; 2];

fn overlap(a: Segment, b: Segment) -> bool {
    a[0] <= b[1] && b[0] <= a[1]
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Block {
    z: Segment,
    rect: Rect<i32>,
}

fn okay(blocks: &[Block], i: usize) -> bool {
    // check if block i intersects any other block with overlapping z
    for (j, block) in blocks.iter().enumerate() {
        if i != j && blocks[i].rect.intersects(&block.rect) && overlap(blocks[i].z, block.z) {
            return false;
        }
    }
    true
}
fn move_down(blocks: Vec<Block>) -> (Vec<Block>, i32) {
    let mut cnt = 0;
    let mut ans = blocks.clone();
    for i in 0..ans.len() {
        //take this block and try to move it down
        let mut modified = false;
        loop {
            if ans[i].z[0] == 1 {
                break;
            }
            ans[i].z[0] -= 1;
            ans[i].z[1] -= 1;
            if !okay(&ans, i) {
                ans[i].z[0] += 1;
                ans[i].z[1] += 1;
                break;
            } else {
                modified = true;
            }
        }
        if modified {
            cnt += 1;
        }
    }
    (ans, cnt)
}
fn main() {
    let mut blocks: Vec<Block> = include_str!("../input/input.txt")
        .split("\n")
        .map(|str| {
            let (point1, point2) = str.split_once("~").unwrap();
            let parse =
                |s: &str| -> Vec<i32> { s.split(",").map(|x| x.parse().unwrap()).collect() };
            let (a, b) = (parse(point1), parse(point2));
            Block {
                z: [a[2], b[2]],
                rect: Rect::new(coord! { x: a[0], y: a[1] }, coord! { x: b[0], y: b[1] }),
            }
        })
        .collect();
    blocks.sort_by_key(|p| p.z[0]);
    blocks = move_down(blocks).0;
    let ans = (0..blocks.len())
        .map(|i| {
            let without_i: Vec<Block> = blocks
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, b)| *b)
                .collect();
            move_down(without_i.clone()).1
        })
        .sum::<i32>();
    println!("{ans}");
}
