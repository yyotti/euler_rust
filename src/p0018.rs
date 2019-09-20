//! [Problem 18](https://projecteuler.net/problem=18)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2018))

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::iter::repeat;

pub struct Solver;

const NUMS: &[&[usize]] = &[
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

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve(NUMS)
    }
}

fn solve(triangle: &[&[usize]]) -> i64 {
    // ダイクストラ法でやってみる。
    // 頂点を始点、底辺の各マスを終点として、最もコストがかかる点を求めればよい。
    // 今回は経路自体は問題にしていないので省略する。
    let s = triangle[0][0];

    let mut d = HashMap::new();
    d.insert((0, 0), s);
    let mut q = BinaryHeap::from(vec![(s, (0, 0))]);

    while q.len() > 0 {
        let (_, (x, y)) = q.pop().unwrap();
        vec![(x + 1, y), (x + 1, y + 1)]
            .iter()
            .filter(|(i, j)| i < &triangle.len() && j < &triangle[*i].len())
            .for_each(|&(i, j)| {
                let alt = match d.get(&(x, y)) {
                    Some(d) => d + triangle[i][j],
                    _ => triangle[i][j],
                };
                if &alt > d.get(&(i, j)).unwrap_or(&0) {
                    d.insert((i, j), alt);
                    q.push((alt, (i, j)));
                }
            });
    }
    *d.iter()
        .filter_map(|(k, v)| {
            if k.0 == triangle.len() - 1 {
                Some(v)
            } else {
                None
            }
        })
        .max()
        .unwrap() as i64
}

#[allow(dead_code)]
fn solve2(triangle: &[&[usize]]) -> i64 {
    // 逆から辿る。
    // 底辺から攻めていき、隣り合う2つの数字の大きい方だけ残した状態にして
    // 1つ上の段の数値に足す。それを繰り返すと最上段との和をとった際に最大値
    // となっている。
    triangle.iter().rev().fold(
        repeat(0).take(triangle.len() + 1).collect::<Vec<usize>>(),
        |acc, row| {
            acc.iter()
                .scan(acc.iter().skip(1), |acc, v| acc.next().map(|a| a.max(v)))
                .zip(row.iter())
                .map(|(a, b)| a + b)
                .collect()
        },
    )[0] as i64
}

#[allow(dead_code)]
fn solve3(triangle: &[&[usize]], p: (usize, usize)) -> i64 {
    // 真面目に全ルートを探索して最大を探す。
    // 問題文にある通り、この問題だからまだ可能なやり方。
    if p.0 > triangle.len() - 1 || p.1 > triangle[p.0].len() - 1 {
        return 0;
    }

    vec![(p.0 + 1, p.1), (p.0 + 1, p.1 + 1)]
        .iter()
        .map(|&(x, y)| solve3(triangle, (x, y)) as u64 + triangle[p.0][p.1] as u64)
        .max()
        .unwrap_or(0) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let triangle: &[&[usize]] = &[
            &[3],          //
            &[7, 4],       //
            &[2, 4, 6],    //
            &[8, 5, 9, 3], //
        ];

        assert_eq!(23, solve(triangle), "solve1");
        assert_eq!(23, solve2(triangle), "solve2");
        assert_eq!(23, solve3(triangle, (0, 0)), "solve3");
    }
}
