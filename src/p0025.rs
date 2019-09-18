//! [Problem 25](https://projecteuler.net/problem=25)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2025))

use super::common::sum_string_int;

pub struct Solver;

const NUM: usize = 1_000;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUM)
    }
}

fn solve(n: usize) -> u64 {
    // 1000桁は普通の数値型では表せないので、文字列演算でやる
    let mut a1 = String::from("1");
    let mut a2 = String::from("1");
    let mut i = 1;
    while a1.len() < n {
        i += 1;
        let tmp = a2.clone();
        a2 = sum_string_int(&a1, &a2);
        a1 = tmp;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let ts = vec![
            (1, 1),  //
            (2, 7),  //
            (3, 12), //
            (4, 17), //
            (5, 21), //
        ];
        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }
}
