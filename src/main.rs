#[macro_use]
extern crate clap;

use clap::Arg;
use euler_rust::*;

fn main() {
    let app = app_from_crate!().arg(
        Arg::with_name("number")
            .help("Problem number (positive integer). If omitted, show all answers.")
            .short("n")
            .long("num")
            .required(true)
            .multiple(true)
            .takes_value(true),
    );

    let matches = app.get_matches();
    match values_t!(matches, "number", u32) {
        Ok(nums) => {
            for n in nums {
                match solve(n) {
                    Some(a) => println!("{:0>4}: {}", n, a),
                    _ => println!("{:0>4}: Cannot solve", n),
                }
            }
        }
        Err(e) => println!("{}", e),
    };
}

fn solve(n: u32) -> Option<i64> {
    let ans = match n {
        1 => p0001::solve(p0001::MAX_NUM),
        2 => p0002::solve(p0002::MAX_NUM),
        3 => p0003::solve(p0003::NUM),
        4 => p0004::solve(p0004::DIGIT_LEN),
        5 => p0005::solve(p0005::NUM),
        6 => p0006::solve(p0006::MAX_NUM),
        7 => p0007::solve(p0007::CNT),
        8 => p0008::solve(p0008::CNT, p0008::NUMS),
        9 => p0009::solve(p0009::MAX_NUM),
        10 => p0010::solve(p0010::MAX_NUM),
        11 => p0011::solve(p0011::GRID),
        12 => p0012::solve(p0012::CNT),
        13 => p0013::solve(p0013::NUMS),
        14 => p0014::solve(p0014::MAX_NUM),
        15 => p0015::solve(p0015::SIZE),
        16 => p0016::solve(p0016::POW),
        17 => p0017::solve(p0017::MAX_NUM),
        18 => p0018::solve(p0018::TRIANGLE),
        19 => p0019::solve(p0019::FROM_YM, p0019::TO_YM),
        20 => p0020::solve(p0020::N),
        21 => p0021::solve(p0021::NUM),
        22 => p0022::solve(p0022::FILE_PATH),
        23 => p0023::solve(p0023::MAX_SUM),
        24 => p0024::solve(p0024::NUMS, p0024::N),
        25 => p0025::solve(p0025::DIGITS),
        26 => p0026::solve(p0026::MAX_NUM),
        27 => p0027::solve(p0027::A, p0027::B),
        28 => p0028::solve(p0028::SIZE),
        29 => p0029::solve(p0029::A, p0029::B),
        30 => p0030::solve(p0030::DIGITS),
        31 => p0031::solve(p0031::N, p0031::COINS),
        32 => p0032::solve(p0032::N),
        33 => p0033::solve(),
        34 => p0034::solve(),
        35 => p0035::solve(p0035::MAX_NUM),
        36 => p0036::solve(p0036::MAX_NUM),
        37 => p0037::solve(),
        38 => p0038::solve(),
        _ => return None,
    };

    Some(ans)
}
