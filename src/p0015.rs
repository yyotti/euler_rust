//! [Problem 15](https://projecteuler.net/problem=15)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2015))

use super::common::gcd;

pub struct Solver;

const NUM: u64 = 20;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUM)
    }
}

fn solve(input: u64) -> u64 {
    // 真面目にやるなら全ルートを探索することだが、面倒なので公式を使う。
    // ただし、普通に公式通りにやると途中でオーバーフローする可能性が高いので、
    // 手書きで解く時のように先に約分をしきってから積をとる。

    let mut ns: Vec<u64> = (input+1..input*2+1).rev().collect();
    for mut r in (2..input+1).rev() {
        while r > 1 {
            match ns.iter().enumerate().map(|(j, &n)| (j, n, gcd(n, r))).find(|(_, _, k)| k > &1) {
                Some((j, n, k)) => {
                    ns[j] = n / k;
                    r /= k;
                },
                None => (),
            };
        }
    }
    ns.iter().fold(1, |acc, n| acc * n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let ts = vec![
            (1, 2),
            (2, 6),
            (3, 20),
            (4, 70),
            (5, 252),
        ];

        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }
}
