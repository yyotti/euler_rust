//! [Problem 38](https://projecteuler.net/problem=38)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2038))

use super::common;
use std::collections::HashSet;

pub fn solve() -> i64 {
    // 問題の通りにやる。
    //
    // 最大値が指定されていないが、例えば 12345に対して1～を掛けていく場合、
    //   12345 x 1 = 12345
    //   12345 x 2 = 24690
    // となり、積の桁数は合計10桁となってしまう。
    // a x n で n > 1 であるから、9桁のパンデジタル数になるためにはaは最大でも
    // 4桁でなければならない。

    (1..9999)
        .filter_map(|a| {
            let mut ds = vec![];
            let mut i = 0;
            while ds.len() < 9 {
                i += 1;
                ds.append(&mut common::digits(i * a as u64));
            }

            if ds.len() > 9 {
                return None;
            }

            let ds_set = ds.iter().cloned().collect::<HashSet<u64>>();
            if ds_set.len() == 9 && !ds_set.contains(&0) {
                Some(common::digits_to_num(
                    &ds.iter().map(|&d| d as usize).collect::<Vec<usize>>(),
                ))
            } else {
                None
            }
        })
        .max()
        .unwrap() as i64
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_solve() {
        // No tests
    }
}
