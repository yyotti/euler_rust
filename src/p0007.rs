//! [Problem 7](https://projecteuler.net/problem=7)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%207))

use super::common::primes;

pub struct Solver;

const NUM: usize = 10001;

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve(NUM)
    }
}

fn solve(input: usize) -> i64 {
    primes().skip(input - 1).next().unwrap() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let ts = vec![
            (1, 2),  //
            (2, 3),  //
            (3, 5),  //
            (4, 7),  //
            (5, 11), //
            (6, 13), //
        ];
        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }
}
