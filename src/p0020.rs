//! [Problem 20](https://projecteuler.net/problem=20)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2020))

use super::common::multi;

pub struct Solver;

const NUM: u64 = 100;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUM)
    }
}

fn solve(input: u64) -> u64 {
    // n!も2^xと同様にnが大きくなるとオーバーフローするので、手動掛け算でやる。
    (1..=input)
        .fold(vec![1], |acc, n| multi(&acc, n))
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(27, solve(10));
    }
}
