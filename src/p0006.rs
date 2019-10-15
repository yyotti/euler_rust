//! [Problem 6](https://projecteuler.net/problem=6)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%206))

pub const MAX_NUM: usize = 100;

pub fn solve(input: usize) -> i64 {
    // 素直にやるなら
    //   1^2 + 2^2 + ... + n^2
    // と
    //   (1 + 2 + ... + n)^2
    // を計算して差をとるのだが、
    //   a(n) = (Σ(n))^2 - Σ(n^2)
    // を求めて代入する方が早いし楽。
    //
    //   Σ(n) = n(n+1)/2
    // より
    //   (Σ(n))^2 = (n(n+1)/2)^2
    //             = ((n(n+1))^2)/4
    //             = (n^2)((n+1)^2)/4
    //             = (n^2)(n^2 + 2n + 1)/4
    //             = (n^4 + 2n^3 + n^2)/4
    // で、
    //   Σ(n^2) = n(n+1)(2n+1)/6
    //           = (2n^3 + 3n^2 + n)/6
    // であるから、
    //   a(n) = (n^4 + 2n^3 + n^2)/4 - (2n^3 + 3n^2 + n)/6
    //        = 3(n^4 + 2n^3 + n^2)/12 - 2(2n^3 + 3n^2 + n)/12
    //        = (3(n^4 + 2n^3 + n^2) - 2(2n^3 + 3n^2 + n))/12
    //        = ((3n^4 + 6n^3 + 3n^2) - (4n^3 + 6n^2 + 2n))/12
    //        = (3n^4 + 2n^3 - 3n^2 - 2n)/12
    //        = n(3n^3 + 2n^2 - 3n - 2)/12
    //        = n(n+1)(3n^2 - n - 2)/12
    //        = n(n+1)(n-1)(3n+2)/12
    (input * (input + 1) * (input - 1) * (3 * input + 2) / 12) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(2640, solve(10))
    }
}
