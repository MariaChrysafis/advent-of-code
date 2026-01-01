use std::cmp;

fn union_intervals(mut vec: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    vec.sort();
    let mut ans = Vec::new();
    for (x, y) in vec {
        let len = ans.len();
        if len == 0 {
            ans.push((x, y));
            continue;
        }
        if ans[len - 1].1 < x {
            ans.push((x, y));
        } else {
            ans[len - 1].1 = cmp::max(ans[len - 1].1, y);
        }
    }
    ans
}
fn main() {
    let ans: i64 = union_intervals(
        include_str!("../input/input.txt")
            .split_once("\n\n")
            .unwrap()
            .0
            .split("\n")
            .map(|x| {
                let (l, r) = x.split_once("-").unwrap();
                return (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap());
            })
            .collect(),
    )
    .iter()
    .map(|pr| pr.1 - pr.0 + 1)
    .sum();
    print!("{ans}");
}
