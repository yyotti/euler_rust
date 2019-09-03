//! [Problem 16](https://projecteuler.net/problem=16)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2016))

pub struct Solver;

const NUM: u64 = 1000;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUM)
    }
}

fn solve(input: u64) -> u64 {
    // 2^x をそのまま計算できれば早いが、x=63あたりで限界がくる。
    // 掛け算を手動で実装し、各桁をVecに詰めたものを作ってみる。
    (1..=input)
        .fold(vec![1], |acc, _| multi(&acc, 2))
        .iter()
        .sum()
}

fn multi(x: &Vec<u64>, y: u64) -> Vec<u64> {
    if x.is_empty() {
        return vec![0];
    }

    let (mut ds, mut c) = x.iter().rev().fold((vec![], 0), |(mut ds, c), d| {
        let z = d * y + c;
        ds.push(z % 10);
        (ds, z / 10)
    });
    while c > 0 {
        ds.push(c % 10);
        c /= 10;
    }
    ds.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi() {
        let ts = vec![
            (vec![], 10, vec![0]),
            (vec![2], 1, vec![2]),
            (vec![3], 3, vec![9]),
            (vec![3], 4, vec![1, 2]),
            (vec![2, 0], 4, vec![8, 0]),
            (vec![2, 0], 5, vec![1, 0, 0]),
            (vec![1, 2, 3], 17, vec![2, 0, 9, 1]),
            (vec![9, 9, 9], 999, vec![9, 9, 8, 0, 0, 1]),
        ];

        for (in_x, in_y, expected) in ts {
            assert_eq!(
                expected,
                multi(&in_x, in_y),
                "{} x {}",
                &in_x.iter().fold(0, |acc, d| acc * 10 + d),
                in_y
            );
        }
    }

    #[test]
    fn test_solve() {
        let ts = vec![
            (1, 2),
            (2, 4),
            (3, 8),
            (4, 7),
            (5, 5),
            (6, 10),
            (7, 11),
            (8, 13),
            (9, 8),
            (10, 7),
            (11, 14),
            (12, 19),
            (13, 20),
            (14, 22),
            (15, 26),
        ];

        for (input, expected) in ts {
            assert_eq!(expected, solve(input), "2^{}", input);
        }
    }
}
