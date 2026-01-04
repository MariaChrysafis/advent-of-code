use good_lp::{Expression, Solution, SolverModel, Variable, default_solver};

fn main() {
    let ans: i32 = include_str!("../input/input.txt")
        .split("\n")
        .map(|line| {
            let a: Vec<&str> = line.split(" ").collect();
            let sz: usize = a[0][1..a[0].len() - 1].len();
            let positions: Vec<Vec<i32>> = a[1..a.len() - 1]
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
            let variables: Vec<Variable> = vars
                .iter_variables_with_def()
                .map(|(variable, _)| variable)
                .collect();
            let mut problem = vars
                .clone()
                .minimise(variables.iter().sum::<Expression>())
                .using(default_solver);
            (0..end.len()).for_each(|ind| {
                problem.add_constraint(
                    variables
                        .iter()
                        .enumerate()
                        .map(|(i, variable)| *variable * positions[i][ind])
                        .sum::<Expression>()
                        .eq(end[ind] as i32),
                );
            });
            let solution = problem.solve().unwrap();
            vars.iter_variables_with_def()
                .map(|(var, _)| solution.value(var).round() as i32)
                .sum::<i32>()
        })
        .sum();
    println!("{ans}");
}
