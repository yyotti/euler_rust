#[macro_use]
extern crate clap;
extern crate chrono;

use clap::Arg;
use clap::ErrorKind;
use euler_rust::*;

fn main() {
    let solvers: Vec<Box<Solver<_>>> = vec![
        Box::new(p0001::Solver),
        Box::new(p0002::Solver),
        Box::new(p0003::Solver),
        Box::new(p0004::Solver),
        Box::new(p0005::Solver),
        Box::new(p0006::Solver),
        Box::new(p0007::Solver),
        Box::new(p0008::Solver),
        Box::new(p0009::Solver),
        Box::new(p0010::Solver),
        Box::new(p0011::Solver),
        Box::new(p0012::Solver),
        Box::new(p0013::Solver),
        Box::new(p0014::Solver),
        Box::new(p0015::Solver),
        Box::new(p0016::Solver),
        Box::new(p0017::Solver),
        Box::new(p0018::Solver),
        Box::new(p0019::Solver),
        Box::new(p0020::Solver),
        Box::new(p0021::Solver),
        Box::new(p0022::Solver),
        Box::new(p0023::Solver),
    ];

    let app = app_from_crate!().arg(
        Arg::with_name("number")
            .help("Problem number (positive integer). If omitted, show all answers.")
            .short("n")
            .long("num")
            .multiple(true)
            .takes_value(true),
    );

    let matches = app.get_matches();
    match values_t!(matches, "number", usize) {
        Ok(ref nums) if (|| nums.iter().all(|&n| n > 0 && n <= solvers.len()))() => {
            for n in nums {
                println!("{:0>4}: {}", n, solvers[n - 1].solve());
            }
        }
        Err(ref e) if e.kind == ErrorKind::ArgumentNotFound => {
            solvers
                .iter()
                .enumerate()
                .for_each(|(i, s)| println!("{:0>4}: {}", i + 1, s.solve()));
        }
        Err(e) => println!("{}", e),
        _ => println!("{}", "Cannot ..."),
    };
}
