//! [Problem 5](https://projecteuler.net/problem=5)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%205))

pub struct Solver;

use super::common;
use std::collections::HashMap;

const NUM: u64 = 20;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUM)
    }
}

fn solve(input: u64) -> u64 {
    (1..input + 1)
        .flat_map(common::prime_factors)
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k, *acc.get(&k).unwrap_or(&0).max(&v));
            acc
        })
        .iter()
        .fold(1, |acc, (k, v)| acc * k.pow(*v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_smallest_mutiple() {
        assert_eq!(2520, solve(10))
    }
}
