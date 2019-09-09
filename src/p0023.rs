//! [Problem 23](https://projecteuler.net/problem=23)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2023))

use super::common::get_prime_factor_sums;
use std::collections::HashMap;

pub struct Solver;

const MAX_SUM: u64 = 28123;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(MAX_SUM)
    }
}

fn solve(input: u64) -> u64 {
    // エラトステネスの篩の要領でやれると思う
    let abundant_numbers: HashMap<_, _> = get_prime_factor_sums(input)
        .into_iter()
        .filter(|(i, f)| i < f)
        .collect();
    (1..=input).map(|i| {
        match abundant_numbers.iter().find(|(&n, _)| i > n && abundant_numbers.contains_key(&(i - n))) {
            Some(_) => 0,
            _ => i,
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(2766, solve(100));
    }
}
