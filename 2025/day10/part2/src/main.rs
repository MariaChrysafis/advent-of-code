use good_lp::{Expression, Solution, SolverModel, default_solver};

fn main() {
    let ans: i32 = include_str!("../input/input.txt")
        .split("\n")
        .map(|line| {
            let a: Vec<&str> = line.split(" ").collect();
            let sz: usize = a[0][1..a[0].len() - 1].len();
            let positions: Vec<Vec<usize>> = a[1..a.len() - 1]
                .to_vec()
                .iter()
                .map(|str| {
                    let arr = &mut vec![0; sz];
                    str[1..str.len() - 1]
                        .split(',')
                        .for_each(|str: &str| arr[str.parse::<usize>().unwrap()] = 1);
                    arr.clone()
                })
                .collect();
            let end: Vec<usize> = a[a.len() - 1][1..a[a.len() - 1].len() - 1]
                .split(",")
                .map(|i| i.parse::<usize>().unwrap())
                .collect();
            let mut vars = good_lp::variables!();
            for _ in positions.clone() {
                vars.add(good_lp::variable().min(0).integer());
            }
            let mut expression: Expression = 0.into();
            for (var, _) in vars.clone().iter_variables_with_def() {
                expression += var;
            }
            let mut problem = vars.clone().minimise(expression).using(default_solver);
            for (ind, _) in end.iter().enumerate() {
                let mut expr: Expression = 0.into();
                for (i, (var, _)) in vars.clone().iter_variables_with_def().enumerate() {
                    if positions[i][ind] == 1 {
                        expr += var;
                    }
                }
                problem.add_constraint(expr.eq(end[ind] as i32));
            }
            let solution = problem.solve().unwrap();
            let mut ans = 0;
            for (var, _) in vars.iter_variables_with_def() {
                ans += solution.value(var).round() as i32;
            }
            println!("{}", ans);
            ans
        })
        .sum();
    println!("{}", ans);
}
