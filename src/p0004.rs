//! [Problem 4](https://projecteuler.net/problem=4)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%204))

use super::common::digits;

pub const DIGIT_LEN: usize = 3;

pub fn solve(input: usize) -> i64 {
    let m = input as u32 - 1;
    let min = 10u64.pow(m);
    let max = 10u64.pow(m + 1);
    (min..max)
        .rev()
        .flat_map(|n| (min..(n + 1)).rev().map(move |m| n * m))
        .filter(|&l| is_palindrome(digits(l).as_slice()))
        .max()
        .unwrap() as i64
}

fn is_palindrome<T: std::cmp::Eq>(arr: &[T]) -> bool {
    match arr {
        [] => true,
        [_] => true,
        ds if ds[0] == ds[ds.len() - 1] => is_palindrome(&ds[1..ds.len() - 1]),
        _ => false,
    }
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

    #[test]
    fn test_is_palindrome() {
        let ts = vec![
            (vec![], true),            //
            (vec![0], true),           //
            (vec![9], true),           //
            (vec![1, 9], false),       //
            (vec![2, 2], true),        //
            (vec![1, 2, 3], false),    //
            (vec![3, 2, 3], true),     //
            (vec![1, 0, 1, 2], false), //
            (vec![2, 0, 1, 2], false), //
            (vec![2, 1, 1, 2], true),  //
        ];
        for (input, expected) in ts {
            assert_eq!(expected, is_palindrome(input.as_slice()), "{:?}", input)
        }
    }
}
