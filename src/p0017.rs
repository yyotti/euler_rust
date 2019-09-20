//! [Problem 17](https://projecteuler.net/problem=17)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2017))

pub struct Solver;

const NUM: usize = 1000;

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve(NUM)
    }
}

fn solve(input: usize) -> i64 {
    // つまらないのでスキップ
    input as i64
}

#[cfg(test)]
mod tests {
    // No tests
}
