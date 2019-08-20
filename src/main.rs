use euler_rust::*;

fn main() {
    let solvers: Vec<Box<Solver<_>>> = vec![
        Box::new(p0001::Solver),
        Box::new(p0002::Solver),
        Box::new(p0003::Solver),
    ];

    for s in solvers {
        println!("{}", s.solve());
    }
}
