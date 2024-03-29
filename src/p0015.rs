//! [Problem 15](https://projecteuler.net/problem=15)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2015))

use super::common::prime_factors;
use std::collections::HashMap;

pub const SIZE: usize = 20;

pub fn solve(input: usize) -> i64 {
    // 分子を素因数分解
    let ns = (input + 1..=input * 2)
        .flat_map(|i| prime_factors(i as u64))
        .fold(HashMap::new(), |mut acc, (p, e)| {
            match acc.remove(&p) {
                Some(k) => acc.insert(p, k + e),
                _ => acc.insert(p, e),
            };
            acc
        });

    // 分母を素因数分解しつつ分子の要素から引いて残りの積をとる
    (2..=input)
        .flat_map(|i| prime_factors(i as u64))
        .fold(ns, |mut acc, (p, e)| match acc.remove(&p) {
            Some(k) if k != e => {
                acc.insert(p, k - e);
                acc
            }
            _ => acc,
        })
        .iter()
        .map(|(&p, &e)| p.pow(e as u32))
        .product::<u64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let ts = vec![
            (1, 2),   //
            (2, 6),   //
            (3, 20),  //
            (4, 70),  //
            (5, 252), //
        ];

        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }
}
