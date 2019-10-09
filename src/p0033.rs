//! [Problem 33](https://projecteuler.net/problem=33)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2033))

use super::common::gcd;

pub struct Solver;

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve()
    }
}

fn solve() -> i64 {
    // 分数を a/b とする。
    // 1より小さく、分母分子がともに2桁である必要があるので、
    //   a, b >= 10 かつ a < b
    // つまり
    //   a >= 10, b > 10, a < b
    // である。
    // また、30/50 = 3/5 のような場合は除くらしいので、a,bともに10の倍数になる
    // 場合は考えなくてよいことになるが、そもそもa,bのどちらかが10の倍数ならば
    // 約分できた際に分母に0がくる（10/21=0/2のような）ので、10の倍数の場合は
    // スキップできる。

    let (_, b) = (11..=99)
        .clone()
        .flat_map(|b| {
            (10..b).filter_map(move |a| {
                if a % 10 == 0 || b % 10 == 0 {
                    return None;
                }

                let (p, q) = divide_mistake(a, b);
                if p >= 10 {
                    return None;
                }

                let frac = divide(a, b);
                if frac != divide(p, q) {
                    return None;
                }

                Some(frac)
            })
        })
        .fold((1, 1), |(p, q), (a, b)| divide(p * a, q * b));
    b as i64
}

fn divide(a: usize, b: usize) -> (usize, usize) {
    let d = gcd(a, b);
    (a / d, b / d)
}

fn divide_mistake(a: usize, b: usize) -> (usize, usize) {
    let p = (a / 10, a % 10);
    let q = (b / 10, b % 10);

    if p == q || (p.1, p.0) == q {
        // 同じ数字で構成されている
        return (1, 1);
    }

    if p.0 == q.0 {
        (p.1, q.1)
    } else if p.1 == q.0 {
        (p.0, q.1)
    } else if p.0 == q.1 {
        (p.1, q.0)
    } else if p.1 == q.1 {
        (p.0, q.0)
    } else {
        (a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        // No tests
    }

    #[test]
    fn test_divide() {
        let ts = vec![
            ((1, 1), (1, 1)),
            ((1, 2), (1, 2)),
            ((2, 2), (1, 1)),
            ((1, 3), (1, 3)),
            ((2, 3), (2, 3)),
            ((4, 3), (4, 3)),
            ((5, 3), (5, 3)),
            ((6, 3), (2, 1)),
            ((12, 16), (3, 4)),
        ];
        for ((input_a, input_b), expected) in ts {
            assert_eq!(
                expected,
                divide(input_a, input_b),
                "{}/{}",
                input_a,
                input_b
            );
        }
    }

    #[test]
    fn test_divide_mistake() {
        let ts = vec![
            ((1, 1), (1, 1)),
            ((1, 2), (1, 2)),
            ((2, 2), (1, 1)),
            ((12, 16), (2, 6)),
            ((21, 16), (2, 6)),
            ((12, 61), (2, 6)),
            ((21, 61), (2, 6)),
            ((31, 13), (1, 1)),
            ((11, 13), (1, 3)),
            ((11, 22), (11, 22)),
        ];
        for ((input_a, input_b), expected) in ts {
            assert_eq!(
                expected,
                divide_mistake(input_a, input_b),
                "{}/{}",
                input_a,
                input_b
            );
        }
    }
}
