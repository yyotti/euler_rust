//! [Problem 3](https://projecteuler.net/problem=3)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%203))

use super::common;

pub struct Solver;

const NUM: u64 = 600_851_475_143;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUM)
    }
}

fn solve(input: u64) -> u64 {
    *common::prime_factors(input).keys().max().unwrap_or(&1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_prime_factor() {
        let ts = vec![
            (0, 1),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 2),
            (5, 5),
            (6, 3),
            (7, 7),
            (8, 2),
            (9, 3),
            (10, 5),
            (13195, 29),
        ];
        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }
}
