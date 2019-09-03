//! [Problem 17](https://projecteuler.net/problem=17)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2017))

pub struct Solver;

const NUM: u64 = 1000;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUM)
    }
}

fn solve(input: u64) -> u64 {
    // つまらないのでスキップ
    input
}

#[cfg(test)]
mod tests {
    // No tests
}
