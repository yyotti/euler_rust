//! [Problem 18](https://projecteuler.net/problem=18)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2018))

use std::iter::repeat;

pub struct Solver;

const NUMS: &[&[u64]] = &[
    &[75],                                                         //
    &[95, 64],                                                     //
    &[17, 47, 82],                                                 //
    &[18, 35, 87, 10],                                             //
    &[20, 04, 82, 47, 65],                                         //
    &[19, 01, 23, 75, 03, 34],                                     //
    &[88, 02, 77, 73, 07, 63, 67],                                 //
    &[99, 65, 04, 28, 06, 16, 70, 92],                             //
    &[41, 41, 26, 56, 83, 40, 80, 70, 33],                         //
    &[41, 48, 72, 33, 47, 32, 37, 16, 94, 29],                     //
    &[53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],                 //
    &[70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],             //
    &[91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],         //
    &[63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],     //
    &[04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23], //
];

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUMS)
    }
}

fn solve(triangle: &[&[u64]]) -> u64 {
    // 逆から辿る。
    // 底辺から攻めていき、隣り合う2つの数字の大きい方だけ残した状態にして
    // 1つ上の段の数値に足す。それを繰り返すと最上段との和をとった際に最大値
    // となっている。
    triangle.iter().rev().fold(
        repeat(0u64).take(triangle.len() + 1).collect::<Vec<u64>>(),
        |acc, row| {
            acc.iter()
                .scan(acc.iter().skip(1), |acc, v| acc.next().map(|a| a.max(v)))
                .zip(row.iter())
                .map(|(a, b)| a + b)
                .collect()
        },
    )[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let triangle: &[&[u64]] = &[
            &[3],          //
            &[7, 4],       //
            &[2, 4, 6],    //
            &[8, 5, 9, 3], //
        ];

        assert_eq!(23, solve(triangle));
    }
}
