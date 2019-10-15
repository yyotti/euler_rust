//! [Problem 10](https://projecteuler.net/problem=10)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2010))

use super::common::sieve;

pub const MAX_NUM: usize = 2_000_000;

pub fn solve(input: usize) -> i64 {
    sieve(input).iter().sum::<u64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let ts = vec![
            (1, 0),   //
            (2, 2),   //
            (3, 5),   //
            (4, 5),   //
            (5, 10),  //
            (6, 10),  //
            (7, 17),  //
            (8, 17),  //
            (9, 17),  //
            (10, 17), //
            (11, 28), //
        ];
        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }
}
