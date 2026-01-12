use std::collections::HashMap;

const TARGETS: [&str; 2] = ["dac", "fft"];
type Node = (String, [bool; 2]);
type Adj = HashMap<String, Vec<String>>;

fn dfs(adj: &Adj, (name, flags): &Node, dp: &mut HashMap<Node, i64>) -> i64 {
    if let Some(&v) = dp.get(&(name.clone(), *flags)) {
        return v;
    }
    let result = adj
        .get(name)
        .into_iter()
        .flatten()
        .map(|n| {
            dfs(
                adj,
                &(
                    n.clone(),
                    std::array::from_fn(|i| flags[i] || n == TARGETS[i]),
                ),
                dp,
            )
        })
        .sum();
    dp.insert((name.clone(), *flags), result);
    result
}

fn main() {
    let adj: Adj = include_str!("../../input/input.txt")
        .lines()
        .filter_map(|l| l.split_once(':'))
        .map(|(n, nb)| (n.into(), nb.split_whitespace().map(Into::into).collect()))
        .collect();
    let end = ("out".into(), [true; 2]);
    println!(
        "{}",
        dfs(&adj, &("svr".into(), [false; 2]), &mut [(end, 1)].into())
    );
}
