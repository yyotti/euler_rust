//! [Problem 30](https://projecteuler.net/problem=30)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2030))

pub const DIGITS: usize = 5;

pub fn solve(input: usize) -> i64 {
    // nをk桁の自然数(n>=2)とし、a1,a2,...,akをnの各桁の数字とする。
    //   n = a1*10^(k-1) + a2*10^(k-2) + ... + ak*10^0
    //
    // nの各桁の数字をr乗した数の和をS(n)とすると、
    //   S(n) = a1^r + a2^r + ... + ak^r
    // 各桁の最大値は9であるから、S(n)の最大値は
    //   S(n) = 9^r + 9^r + ... + 9^r
    //        = k * 9^r
    // であるから、nの値がこれを超える場合はS(n)=nになることはない。
    // また、S(n)の最大値がk桁未満の場合も、やはりS(n)=nになることはない。
    (1..)
        .map(|k| {
            (
                10usize.pow(k as u32 - 1),
                (10usize.pow(k as u32) - 1).min(k * 9usize.pow(input as u32)),
            )
        })
        .take_while(|(f, t)| f < t)
        .flat_map(|(f, t)| {
            (f..=t).filter(|&n| {
                (1..)
                    .scan(n, |acc, _| {
                        if *acc == 0 {
                            None
                        } else {
                            let r = *acc % 10;
                            *acc /= 10;
                            Some(r)
                        }
                    })
                    .map(|d| d.pow(input as u32))
                    .sum::<usize>()
                    == n
            })
        })
        .sum::<usize>() as i64
        - 1 // 1^rは除く
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(44, solve(1));
        assert_eq!(0, solve(2));
        assert_eq!(1301, solve(3));
        assert_eq!(19316, solve(4));
    }
}
