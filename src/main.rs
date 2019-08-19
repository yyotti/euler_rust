use euler_rust::Solver;
use euler_rust::p0001;
use euler_rust::p0002;

fn main() {
    let solvers: Vec<Box<Solver<_>>> = vec![
        Box::new(p0001::Solver),
        Box::new(p0002::Solver),
    ];

    for s in solvers {
        println!("{}", s.solve());
    }
}
