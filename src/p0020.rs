//! [Problem 20](https://projecteuler.net/problem=20)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2020))

use super::common::multi;

pub const N: usize = 100;

pub fn solve(input: usize) -> i64 {
    // n!も2^xと同様にnが大きくなるとオーバーフローするので、手動掛け算でやる。
    (1..=input)
        .fold(vec![1], |acc, n| multi(&acc, n as u64))
        .iter()
        .sum::<u64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(27, solve(10));
    }
}
