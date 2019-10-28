//! [Problem 41](https://projecteuler.net/problem=41)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2041))

use super::common;

pub fn solve() -> i64 {
    // パンデジタルになる必要があるので、1～Nまでの整数(1 <= N <= 9)の順列を
    // 求め、それを整数とみなして素数判定を行えばよい。
    //
    // ここで、N=9の場合は1～9の順列になるが、
    //   1+2+3+4+5+6+7+8+9=45
    // であるから、9桁のパンデジタル数は必ず3の倍数になり素数にはならない。
    // よって、Nは最大でも8桁である。
    (1..=8)
        .rev()
        .flat_map(|n| {
            let mut ns = common::permutations((1..=n).collect::<Vec<usize>>().as_slice(), n)
                .into_iter()
                .map(|ds| common::digits_to_num(&ds))
                .collect::<Vec<usize>>();
            ns.sort_by(|a, b| b.cmp(a));
            ns
        })
        .find(|&n| common::is_prime(n as u64))
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
