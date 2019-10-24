//! [Problem 4](https://projecteuler.net/problem=4)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%204))

use super::common;

pub const DIGIT_LEN: usize = 3;

pub fn solve(input: usize) -> i64 {
    let m = input as u32 - 1;
    let min = 10u64.pow(m);
    let max = 10u64.pow(m + 1);
    (min..max)
        .rev()
        .flat_map(|n| (min..(n + 1)).rev().map(move |m| n * m))
        .filter(|&l| common::is_palindrome(common::digits(l).as_slice()))
        .max()
        .unwrap() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let ts = vec![
            (1, 9),    //
            (2, 9009), //
        ];
        for (input, expected) in ts {
            assert_eq!(expected, solve(input), "{}", input)
        }
    }
}
