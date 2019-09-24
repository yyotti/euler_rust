//! [Problem 29](https://projecteuler.net/problem=29)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2029))

use super::common::multi;
use std::collections::HashSet;

pub struct Solver;

const A: usize = 100;
const B: usize = 100;

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve(A, B)
    }
}

fn solve(a_max: usize, b_max: usize) -> i64 {
    // 素直に全パターンを検証する。
    // ただし、最終的に100^100も計算する必要があり普通の数値では桁が足りない
    // ので、手動掛け算で計算する。
    (2..=a_max)
        .flat_map(|a| {
            (2..=b_max).scan(vec![a as u64], move |acc, _| {
                *acc = multi(&acc, a as u64);
                Some(acc.clone())
            })
        })
        .map(|v| v.iter().fold(String::new(), |acc, d| acc + &d.to_string()))
        .collect::<HashSet<String>>()
        .len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(15, solve(5, 5));
    }
}
