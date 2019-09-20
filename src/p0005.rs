//! [Problem 5](https://projecteuler.net/problem=5)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%205))

pub struct Solver;

use super::common::prime_factors;
use std::collections::HashMap;

const NUM: usize = 20;

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve(NUM)
    }
}

fn solve(input: usize) -> i64 {
    (1..input as u64 + 1)
        .flat_map(prime_factors)
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k, *acc.get(&k).unwrap_or(&0).max(&v));
            acc
        })
        .iter()
        .map(|(k, &v)| k.pow(v as u32))
        .product::<u64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(2520, solve(10))
    }
}
