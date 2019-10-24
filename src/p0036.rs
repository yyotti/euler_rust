//! [Problem 36](https://projecteuler.net/problem=36)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2036))

use super::common;

pub const MAX_NUM: usize = 1_000_000;

pub fn solve(max: usize) -> i64 {
    // 2進で回分数になるためには、先頭に0を付加してはいけないことを考えると
    // 末尾が1でなければならない。つまり奇数のみを調べればよい。
    //
    // また、2進より10進の方が回分数になる可能性は低いので、10進→2進の順で
    // チェックする。

    let mut sum = 0;

    for n in (1..max).step_by(2) {
        if !common::is_palindrome(&common::digits(n as u64)) {
            continue;
        }

        let bin = format!("{:b}", n);
        if common::is_palindrome(&bin.as_bytes()) {
            sum += n;
        }
    }

    sum as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(157, solve(100));
    }
}
