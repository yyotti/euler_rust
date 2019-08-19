use euler_rust::Solver;
use euler_rust::p0001;

fn main() {
    let solvers = vec![
        p0001::Solver,
    ];

    for s in solvers {
        println!("{}", s.solve());
    }
}
