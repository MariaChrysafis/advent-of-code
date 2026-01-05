use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver};

const DIMENSIONS: usize = 3;
fn main() {
    let input = include_str!("../input/input.txt")
        .split("\n")
        .map(|str| {
            let (position, direction) = str.split_once("@").unwrap();
            let parse = |s: &str| -> [i64; DIMENSIONS] {
                s.split(",")
                    .map(|x| x.trim().parse().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            };
            [parse(position), parse(direction)]
        })
        .take(3);

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);
    let variables: [Vec<Int>; 2] = std::array::from_fn(|j| {
        (0..DIMENSIONS)
            .map(|i| Int::fresh_const(&ctx, &format!("{j}{i}")))
            .collect()
    });
    for (i, ray) in input.enumerate() {
        let var = Int::fresh_const(&ctx, &format!("a{}", i));
        solver.assert(&var.gt(&Int::from_u64(&ctx, 0))); // t >= 0
        for k in 0..DIMENSIONS {
            let left = &var * ray[1][k] + ray[0][k];
            let right = &var * &variables[1][k] + &variables[0][k];
            solver.assert(&left._eq(&right));
        }
    }
    solver.check();
    let model = solver.get_model().unwrap();
    let ans: i64 = variables[0]
        .iter()
        .map(|pv| model.eval(pv, true).unwrap().as_i64().unwrap())
        .sum();
    println!("ans: {}", ans);
}
