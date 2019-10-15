//! [Problem 26](https://projecteuler.net/problem=26)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2026))

use std::collections::HashMap;

pub const MAX_NUM: usize = 1_000;

pub fn solve(n: usize) -> i64 {
    // 筆算を模倣して循環部を求める。
    //
    // 例として 1/7 の循環部を求めてみる。
    //   1/7 = 0.x (1)
    //   10/7 = 0.1x (3)
    //   30/7 = 0.14x (2)
    //   20/7 = 0.142x (6)
    //   60/7 = 0.1428x (4)
    //   40/7 = 0.14285x (5)
    //   50/7 = 0.142857x (1)
    // ここで最初に戻ったので、循環部は 142857 となる。

    (2..n)
        .map(|d| (d, cycle_len(d)))
        .max_by(|(_, l1), (_, l2)| l1.cmp(l2))
        .map(|t| t.0)
        .unwrap() as i64
}

fn cycle_len(d: usize) -> usize {
    let mut l = 0usize;
    let mut cache = HashMap::new();
    let mut k = 1;
    while k > 0 && !cache.contains_key(&k) {
        cache.insert(k, l);
        l += 1;
        k = k * 10 % d;
    }

    if k == 0 {
        return 0;
    }

    l - cache.get(&k).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_len() {
        let ts = vec![
            (2, 0),
            (3, 1),
            (4, 0),
            (5, 0),
            (6, 1),
            (7, 6),
            (8, 0),
            (9, 1),
            (10, 0),
        ];
        for (input, expected) in ts {
            assert_eq!(expected, cycle_len(input), "input={}", input);
        }
    }

    #[test]
    fn test_solve() {
        assert_eq!(7, solve(10));
    }
}
