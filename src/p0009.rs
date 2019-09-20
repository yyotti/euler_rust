//! [Problem 9](https://projecteuler.net/problem=9)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%209))

use super::common::gcd;

pub struct Solver;

const MAX: usize = 1000;

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve()
    }
}

fn solve() -> i64 {
    // ピタゴラス数の一般項は、互いに素な正の整数 m, n (m > n, m - n は奇数) を用いて、
    //   a = m^2 - n^2, b = 2mn, c = m^2 + n^2
    // で求められる。
    // ただし、これで求められるのは原始ピタゴラス数なので、1000の約数を探し出して
    // そのabcを定数倍すればよい。
    (2..)
        .flat_map(|m| {
            let start = if m % 2 == 0 { 1 } else { 2 };
            (start..m).step_by(2).filter_map(move |n| {
                if (m - n) % 2 == 1 && gcd(m as u64, n as u64) == 1 {
                    Some((m * m - n * n, 2 * m * n, m * m + n * n))
                } else {
                    None
                }
            })
        })
        .take_while(|&(a, b, c)| a < MAX || b < MAX || c < MAX)
        .filter_map(|(a, b, c)| {
            if MAX % (a + b + c) == 0 {
                let r = MAX / (a + b + c);
                Some(a * b * c * r * r * r)
            } else {
                None
            }
        })
        .max().unwrap_or(0) as i64
}

#[cfg(test)]
mod tests {
    // No tests
}
