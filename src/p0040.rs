//! [Problem 40](https://projecteuler.net/problem=40)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2040))

use super::common;

pub const INDICES: &[usize] = &[1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];

pub fn solve(input: &[usize]) -> i64 {
    // 数学で求める。
    //
    // チャンパーノウン定数の小数以下第n位が、何桁の整数に含まれているかをまず
    // 考える。
    //   ・1桁の数字は 1～9 で、計9桁ある。
    //   ・2桁の数字は 10～99 で、(99-10+1)*2=180 桁あるので、
    //     99までで計189桁。
    //   ・3桁の数字は 100～999 で、(999-100+1)*3=2700桁あるので、
    //     999までで2889桁。
    //   ...
    //   ・k桁の数字は 10^(k-1)～10^k-1 で、
    //       (10^k-10^(k-1))*k = k*10^(k-1)*(10-1) = 9k*10^(k-1) 桁
    //     あるので、k桁の数字の最大値(10^k-1)までで
    //       Σ(i=1～k) {9i*10^(i-1)} 桁
    //     である。
    // これにより、何桁の数字に含まれているかが分かる。
    //
    // 仮に、第n位がd桁の数字に含まれているとし、d桁の整数が小数第j位から
    // 始まっているとする。この時、
    //   [(n-j)/d]+10^(d-1)
    // で、どの整数に含まれているかが分かる。ただし[x]は床関数とする。
    //
    // 仮に、第n位が整数Nに含まれているとすれば、あとはその整数の何桁目かが
    // 分かればよい。これは
    //   (n-j) mod d
    // で求めることができる。

    input.iter().map(|&i| cn(i)).product::<usize>() as i64
}

#[allow(dead_code)]
fn solve2(input: &[usize]) -> i64 {
    // イテレータを作って、指定されたインデックスの数字の積をとる。
    let mut c = Champernowne::new();
    input
        .iter()
        .zip(input.iter().skip(1))
        .fold(c.nth(input[0]).unwrap(), |acc, (&i1, &i2)| {
            acc * c.nth(i2 - i1 - 1).unwrap()
        }) as i64
}

struct Champernowne {
    n: usize,
    ds: Vec<usize>,
    idx: usize,
}

impl Champernowne {
    fn new() -> Champernowne {
        Champernowne {
            n: 0,
            ds: vec![0],
            idx: 0,
        }
    }
}

impl Iterator for Champernowne {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.ds.len() {
            self.n += 1;
            self.ds = common::digits(self.n as u64)
                .iter()
                .map(|&d| d as usize)
                .collect::<Vec<usize>>();
            self.idx = 0;
        }

        let res = Some(self.ds[self.idx]);
        self.idx += 1;
        res
    }
}

fn cn(n: usize) -> usize {
    let (d, j) = (1..)
        .scan(0, |acc, k| {
            let before = *acc;
            *acc += 9 * k * 10usize.pow(k as u32 - 1);
            Some((k, *acc, before + 1))
        })
        .find_map(|(k, l, m)| if l >= n { Some((k, m)) } else { None })
        .unwrap();

    let m = (n - j) / d + 10usize.pow(d as u32 - 1);

    common::digits(m as u64)[(n - j) % d] as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(768, solve(&[2, 4, 6, 8, 10, 12, 15]));
    }

    #[test]
    fn test_champernowne_new() {
        let actual = Champernowne::new();
        assert_eq!(0, actual.n);
        assert_eq!(vec![0], actual.ds);
        assert_eq!(0, actual.idx);
    }

    #[test]
    fn test_champernowne_next() {
        let digits = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 1, 1, 1, 2, 1, 3, 1, 4, 1, 5,
        ];

        let mut c = Champernowne {
            n: 0,
            ds: vec![0],
            idx: 0,
        };

        for expected in digits {
            assert_eq!(Some(expected), c.next());
        }
    }

    #[test]
    fn test_cn() {
        let ts = vec![
            (1, 1),   //
            (2, 2),   //
            (3, 3),   //
            (4, 4),   //
            (5, 5),   //
            (12, 1),  //
            (234, 4), //
        ];

        for (input, expected) in ts {
            assert_eq!(expected, cn(input));
        }
    }
}
