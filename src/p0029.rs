//! [Problem 29](https://projecteuler.net/problem=29)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2029))

use super::common::gcd;
use super::common::multi;
use super::common::prime_factors;
use std::collections::HashSet;

pub const A: usize = 100;
pub const B: usize = 100;

pub fn solve(a_max: usize, b_max: usize) -> i64 {
    // 無理に計算しないようにする。
    // 具体的にべき乗計算をするのではなく、m^nを(m, n)で表現する。
    //
    // ただし、
    //   m^n = (m^k)^(n/k)
    // の性質を考慮する必要がある。例えば
    //   12^4 = (12, 4)
    //   144^2 = (144, 2)
    // であるが、
    //   144 = 12^2
    // であるため
    //   144^2 = (12^2)^2 = (12, 4) = 12^4
    // となり、2つは同じ数なので重複して数えないようにしなければならない。
    // (本問では最大でもa=100であるため、144^2はありえないが)
    //
    // そこで、m^nにおいてまずmを素因数分解する。
    //   m = p1^e1 * p2^e2 * ... * pk^ek
    // と素因数分解できたとして、e1,e2,...,ekが1以外の公約数tをもつ場合、
    //   m = (p1^(e1/t) * p1^(e2/t) * ... * pk^(ek/t))^t
    // と表現できる。
    //   g1 = e1/t, g2=e2/t, ..., gk=ek/t
    // とした場合、
    //   m = (p1^g1 * p1^g2 * ... * pk^gk)^t
    // となるが、g1,g2,...,gkが公約数t'をもつ場合、さらに別の表現が生まれて
    // しまうため、tはe1,e2,...,ekの最大公約数でなければならない。
    (2..=a_max)
        .flat_map(|a| {
            (2..=b_max).map(move |b| {
                let pf = prime_factors(a as u64);
                let k = pf
                    .iter()
                    .map(|(_, &e)| e)
                    .fold(0, |acc, e| gcd(acc, e));
                (
                    pf.iter()
                        .fold(1, |acc, (p, &e)| acc * p.pow(e as u32 / k as u32)),
                    b * k as usize,
                )
            })
        })
        .collect::<HashSet<(u64, usize)>>()
        .len() as i64
}

#[allow(dead_code)]
fn solve2(a_max: usize, b_max: usize) -> i64 {
    // 素直に全パターンを検証する。
    // ただし、最終的に100^100も計算する必要があり普通の数値では桁が足りない
    // ので、手動掛け算で計算する。
    (2..=a_max)
        .flat_map(|a| {
            (2..=b_max).scan(vec![a as u64], move |acc, _| {
                *acc = multi(&acc, a as u64);
                Some(acc.clone())
            })
        })
        .map(|v| v.iter().fold(String::new(), |acc, d| acc + &d.to_string()))
        .collect::<HashSet<String>>()
        .len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(15, solve(5, 5));
        assert_eq!(15, solve2(5, 5));
    }
}
