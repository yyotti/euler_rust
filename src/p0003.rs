//! [Problem 3](https://projecteuler.net/problem=3)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%203))

use super::common::prime_factors;

pub struct Solver;

const NUM: usize = 600_851_475_143;

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve(NUM)
    }
}

fn solve(input: usize) -> i64 {
    *prime_factors(input as u64).keys().max().unwrap_or(&1) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let ts = vec![
            (0, 1),      //
            (1, 1),      //
            (2, 2),      //
            (3, 3),      //
            (4, 2),      //
            (5, 5),      //
            (6, 3),      //
            (7, 7),      //
            (8, 2),      //
            (9, 3),      //
            (10, 5),     //
            (13195, 29), //
        ];
        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }
}
