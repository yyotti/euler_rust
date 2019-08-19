//! [Problem 1](https://projecteuler.net/problem=1)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%201))

pub struct Solver;

const MAX_NUM: u64 = 1000;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(MAX_NUM)
    }
}

fn solve(input: u64) -> u64 {
    (1..input).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_mod3_mod5() {
        let ts = vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 3),
            (5, 3),
            (6, 8),
            (7, 14),
            (8, 14),
            (9, 14),
        ];

        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }
}
