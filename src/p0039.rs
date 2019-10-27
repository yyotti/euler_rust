//! [Problem 39](https://projecteuler.net/problem=39)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2039))

use super::common;
use std::collections::HashMap;

pub const MAX_NUM: usize = 1000;

const MIN_A: usize = 3;
const MIN_B: usize = 4;
const MIN_C: usize = 5;

pub fn solve(input: usize) -> i64 {
    // {a, b, c}が直角三角形を作る整数の組であるから、ピタゴラス数（とその倍数）
    // である。
    // ここで、cが直角三角形の斜辺であるとする。
    //
    //   a + b + c = p
    // となるなら、kを整数として
    //   ka + kb + kc = kp
    // である。
    // つまり、原始ピタゴラス数 {a', b', c'} を順に調べていき、
    //   kc' < 1000
    // を満たす自然数kについて
    //   a' + b' + c' = p1
    //   2a' + 2b' + 2c' = 2p1 = p2
    //   ...
    //   ka' + kb' + kc' = kp1 = pk
    // を数え上げ、最も多くのパターンで生成できるpを見つければよい。
    (2..)
        .flat_map(|m| {
            let start = if m % 2 == 0 { 1 } else { 2 };
            (start..m).step_by(2).filter_map(move |n| {
                if (m - n) % 2 == 1 && common::gcd(m, n) == 1 {
                    Some((m * m - n * n, 2 * m * n, m * m + n * n))
                } else {
                    None
                }
            })
        })
        .take_while(|&(a, b, c)| a < input || b < input || c < input)
        .filter(|&(a, b, c)| a + b + c <= input)
        .fold(HashMap::new(), |mut acc, (a, b, c)| {
            let p = a + b + c;
            (1..)
                .map(|i| p * i)
                .take_while(|&p| p <= input)
                .for_each(|p| {
                    if acc.contains_key(&p) {
                        acc.insert(p, *acc.get(&p).unwrap() + 1);
                    } else {
                        acc.insert(p, 1);
                    }
                });
            acc
        })
        .into_iter()
        .max_by(|c1, c2| c1.1.cmp(&c2.1))
        .unwrap_or((0, 0))
        .0 as i64
}

#[allow(dead_code)]
fn solve2(input: usize) -> i64 {
    // {a, b, c}は直角三角形を作る整数の組であるから、cを斜辺とすると
    //   a^2 + b^2 = c^2
    // である。
    // また、
    //   a + b > c
    // でなければならず、さらに a <= b を仮定してよい。
    //
    // 最小の直角三角形は {3, 4, 5} であるから、
    //   a >= 3
    //   b >= 4
    //   c >= 5
    //   p >= 3 + 4 + 5 = 12
    // である。

    (MIN_A..=(input - MIN_B - MIN_C))
        .flat_map(|a| {
            ((a.max(MIN_B))..=(input - a - MIN_C))
                .flat_map(move |b| (MIN_C..=(input - a - b)).map(move |c| (a, b, c)))
        })
        .filter(|&(a, b, c)| a * a + b * b == c * c)
        .fold(HashMap::new(), |mut acc, (a, b, c)| {
            let p = a + b + c;
            if acc.contains_key(&p) {
                acc.insert(p, *acc.get(&p).unwrap() + 1);
            } else {
                acc.insert(p, 1);
            }
            acc
        })
        .into_iter()
        .max_by(|c1, c2| c1.1.cmp(&c2.1))
        .unwrap_or((0, 0))
        .0 as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(120, solve(150));
        assert_eq!(120, solve2(150));
    }
}
