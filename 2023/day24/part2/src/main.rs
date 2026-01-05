use z3::ast::{Ast, Int};
use z3::{Config, Context, SatResult, Solver};

const DIMENSIONS: usize = 3;
type Ray = [[i64; DIMENSIONS]; 2];
fn main() {
    let input: Vec<Ray> = include_str!("../input/input.txt")
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
        .take(3)
        .collect();

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);
    let (point_variables, direction_variables): (Vec<_>, Vec<_>) = (0..DIMENSIONS)
        .map(|i| {
            (
                Int::fresh_const(&ctx, &format!("p{i}")),
                Int::fresh_const(&ctx, &format!("d{i}")),
            )
        })
        .unzip();
    for i in 0..input.len() {
        let var = Int::fresh_const(&ctx, &format!("a{}", i));
        solver.assert(&var.gt(&Int::from_u64(&ctx, 0))); // t >= 0
        for k in 0..DIMENSIONS {
            let left = &var * input[i][1][k] + input[i][0][k];
            let right = &var * &direction_variables[k] + &point_variables[k];
            solver.assert(&left._eq(&right));
        }
    }
    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let ans: i64 = point_variables
                .iter()
                .map(|v| model.eval(v, true).unwrap().as_i64().unwrap())
                .sum();
            println!("ans: {}", ans);
        }
        SatResult::Unknown => {
            println!("unknown")
        }
        SatResult::Unsat => {
            println!("No solution found");
        }
    }
}
