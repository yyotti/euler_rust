#[macro_use]
extern crate clap;

use euler_rust::*;
use clap::Arg;
use clap::ErrorKind;

fn main() {
    let solvers: Vec<Box<Solver<_>>> = vec![
        Box::new(p0001::Solver),
        Box::new(p0002::Solver),
        Box::new(p0003::Solver),
        Box::new(p0004::Solver),
        Box::new(p0005::Solver),
        Box::new(p0006::Solver),
    ];

    let app = app_from_crate!()
        .arg(
            Arg::with_name("number")
                .help("Problem number (positive integer). If omitted, show all answers.")
                .short("n")
                .long("num")
                .multiple(true)
                .takes_value(true),
        );

    let matches = app.get_matches();
    match values_t!(matches, "number", usize) {
        Ok(ref nums) if (|| nums.iter().all(|&n| n > 0 && n <= solvers.len()))() => for n in nums {
            println!("{}", solvers[n-1].solve());
        },
        Err(ref e) if e.kind == ErrorKind::ArgumentNotFound => {
            solvers.iter().for_each(|s| println!("{}", s.solve()));
        },
        Err(e) => println!("{}", e),
        _ => println!("{}", "Cannot ..."),
    };
}
