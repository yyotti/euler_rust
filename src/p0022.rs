//! [Problem 22](https://projecteuler.net/problem=22)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2022))

use std::fs;

pub const FILE_PATH: &str = "data/p022_names.txt";

pub fn solve(input: &str) -> i64 {
    let mut names = load_names(input);
    names.sort();
    names.iter().enumerate().fold(0, |acc, (i, name)| {
        acc + (i as i64 + 1) * calc_name_score(name) as i64
    })
}

fn load_names(file: &str) -> Vec<String> {
    match fs::read_to_string(file) {
        Ok(s) => s
            .trim()
            .split(',')
            .map(|name| String::from(&name[1..name.len() - 1]))
            .collect(),
        _ => vec![],
    }
}

fn calc_name_score(s: &str) -> usize {
    s.chars().map(|c| c as usize - 'A' as usize + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_name_score() {
        let ts = vec![
            ("ALICE", 30), //
            ("BOB", 19),   //
            ("COLIN", 53), //
        ];
        for (name, expected) in ts {
            assert_eq!(expected, calc_name_score(name));
        }
    }

    #[test]
    fn test_load_names() {
        assert_eq!(
            vec!["MARY", "PATRICIA", "LINDA", "BARBARA", "ELIZABETH"],
            load_names("./testdata/p022.txt")
        );
    }

    #[test]
    fn test_solve() {
        // 1: BARBARA = 1 * (2 + 1 + 18 + 2 + 1 + 18 + 1) = 43
        // 2: ELIZABETH = 2 * (5 + 12 + 9 + 26 + 1 + 2 + 5 + 20 + 8) = 176
        // 3: LINDA = 3 * (12 + 9 + 14 + 4 + 1) = 120
        // 4: MARY = 4 * (13 + 1 + 18 + 25) = 228
        // 5: PATRICIA = 5 * (16 + 1 + 20 + 18 + 9 + 3 + 9 + 1) = 385
        assert_eq!(952, solve("./testdata/p022.txt"));
    }
}
