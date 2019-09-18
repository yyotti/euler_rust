//! [Problem 25](https://projecteuler.net/problem=25)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2025))

use super::common::sum_string_int;

pub struct Solver;

const NUM: usize = 1_000;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUM)
    }
}

fn solve(n: usize) -> u64 {
    // (参考) https://blog.miz-ar.info/2019/01/fast-fibonacci/
    //
    // フィボナッチ数列には一般項が存在する。
    // sqrt(5) を r5 と表すことにすると、
    //   F(n) = {((1 + r5) / 2)^n - ((1 - r5) / 2)^n} / r5
    //
    // この式は正式ではあるが、下記の方法でも同じ結果が得られる。
    //   F(n) = ROUND(((1 + r5) / 2)^n / r5)
    // ただし ROUND(x) は、xに最も近い整数を返す関数（つまり小数第1位を四捨五入）
    // とする。
    //
    // しかし、本問では桁数のみを考えればよく、具体的な F(n) は求められていない。
    // よって、
    //   F(n) = FLOOR(((1 + r5) / 2)^n / r5)   (FLOOR:切り上げ)
    // もしくは
    //   F(n) = CEIL(((1 + r5) / 2)^n / r5)    (CEIL:切り捨て)
    // としても、桁数に違いは生じない。
    //
    // ここで、
    //   D(n) = FLOOR(LOG(F(n)) + 1)   (LOG:常用対数 log10)
    // とすると、
    //   D(n) ~= LOG(((1 + r5) / 2)^n / r5) + 1
    //         = LOG(((1 + r5) / 2)^n) - LOG(r5) + 1
    //         = n * LOG((1 + r5) / 2) - LOG(5^(1/2)) + 1
    //         = n * (LOG(1 + r5) - LOG(2)) - LOG(5) / 2 + 1
    // これがk桁以上となればよい、つまり
    //   k <= D(n)
    // となる最小の n を見つければよい。
    //   k <= n * (LOG(1 + r5) - LOG(2)) - LOG(5) / 2 + 1
    //   k - 1 + LOG(5) / 2 <= n * (LOG(1 + r5) - LOG(2))
    //   (k - 1 + LOG(5) / 2) / (LOG(1 + r5) - LOG(2)) <= n
    // よって、
    //   CEIL((k + LOG(5) / 2) / (LOG(1 + r5) - LOG(2)))
    // を計算すればよいことになる。
    //
    // ただし、k=1の場合のみうまくいかないので、そこは固定で1を返す。
    if n == 1 {
        return 1;
    }

    let k = n as f64;
    ((k - 1.0 + 5f64.log10() / 2.0) / ((1.0 + 5f64.sqrt()).log10() - 2f64.log10())).ceil() as u64
}

#[allow(dead_code)]
fn solve2(n: usize) -> u64 {
    // 1000桁は普通の数値型では表せないので、文字列演算でやる
    let mut a1 = String::from("1");
    let mut a2 = String::from("1");
    let mut i = 1;
    while a1.len() < n {
        i += 1;
        let tmp = a2.clone();
        a2 = sum_string_int(&a1, &a2);
        a1 = tmp;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let ts = vec![
            (1, 1),  //
            (2, 7),  //
            (3, 12), //
            (4, 17), //
            (5, 21), //
        ];
        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
            assert_eq!(expected, solve2(input));
        }
    }
}
