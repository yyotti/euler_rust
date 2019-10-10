//! [Problem 34](https://projecteuler.net/problem=34)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2034))

use super::common::{digits, fact};

pub struct Solver;

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve()
    }
}

fn solve() -> i64 {
    // nを自然数とし、その桁数をkとする。
    // また、nの各桁の数字をakとして、
    //   f(n) = Σ(l=1～k) al!
    // とする。
    // 0 <= ak <= 9 であることから、f(n)の最大値 f'(n) は
    //   f'(n) = Σ(k=1～) 9! = k*9!
    // である。f'(n) がk桁の自然数の最小値より常に小さくなれば、それ以上のkに
    // ついては f(n)=n となることはない。
    //   k=7の時   nの最小値=1000000   f'(n) = 7*9! = 2540160
    //   k=8の時   nの最小値=10000000   f'(n) = 8*9! = 2903040
    // 以上のことから、n=2540160まで調べればよい。
    //
    // また、k=1の場合は
    //   n=3  f(n)=6
    //   n>=4  f(n)は2桁以上
    // となるので、k>=2の範囲で考えればよい。
    let facts: Vec<u64> = (0..10).map(fact).collect();
    let max = (2..)
        .find(|&n| 10usize.pow(n as u32 - 1) as u64 >= n * facts[9])
        .unwrap_or(1)
        - 1;
    (10..max * facts[9])
        .filter(|&n| n == digits(n).iter().map(|&d| facts[d as usize]).sum())
        .sum::<u64>() as i64
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_solve() {
        // No tests
    }
}
