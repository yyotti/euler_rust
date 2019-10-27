//! [Problem 9](https://projecteuler.net/problem=9)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%209))

use super::common;

pub const MAX_NUM: usize = 1000;

pub fn solve(input: usize) -> i64 {
    // ピタゴラス数の一般項は、互いに素な正の整数 m, n (m > n, m - n は奇数) を用いて、
    //   a = m^2 - n^2, b = 2mn, c = m^2 + n^2
    // で求められる。
    // ただし、これで求められるのは原始ピタゴラス数なので、1000の約数を探し出して
    // そのabcを定数倍すればよい。

    // TODO 1つだけ存在することを確認していない
    common::pythagoras()
        .take_while(|&(a, b, c)| a < input || b < input || c < input)
        .filter_map(|(a, b, c)| {
            if input % (a + b + c) == 0 {
                let r = input / (a + b + c);
                Some(a * b * c * r * r * r)
            } else {
                None
            }
        })
        .max().unwrap_or(0) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(0, solve(100));
        assert_eq!(255000, solve(200));
    }
}
