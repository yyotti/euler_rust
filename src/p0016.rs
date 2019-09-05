//! [Problem 16](https://projecteuler.net/problem=16)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2016))

use super::common::multi;

pub struct Solver;

const NUM: u64 = 1000;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUM)
    }
}

fn solve(input: u64) -> u64 {
    // 2^x をそのまま計算できれば早いが、x=63あたりで限界がくる。
    // 掛け算を手動で実装し、各桁をVecに詰めたものを作ってみる。
    (1..=input)
        .fold(vec![1], |acc, _| multi(&acc, 2))
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let ts = vec![
            (1, 2),
            (2, 4),
            (3, 8),
            (4, 7),
            (5, 5),
            (6, 10),
            (7, 11),
            (8, 13),
            (9, 8),
            (10, 7),
            (11, 14),
            (12, 19),
            (13, 20),
            (14, 22),
            (15, 26),
        ];

        for (input, expected) in ts {
            assert_eq!(expected, solve(input), "2^{}", input);
        }
    }
}
