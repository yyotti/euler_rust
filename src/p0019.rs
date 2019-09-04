//! [Problem 19](https://projecteuler.net/problem=19)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2019))

use chrono::prelude::*;

pub struct Solver;

const FROM: (i32, u32, u32) = (1901, 1, 1);
const TO: (i32, u32, u32) = (2000, 12, 31);

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(
            NaiveDate::from_ymd(FROM.0, FROM.1, FROM.2),
            NaiveDate::from_ymd(TO.0, TO.1, TO.2),
        )
    }
}

fn solve(f: NaiveDate, t: NaiveDate) -> u64 {
    // 1日ずつ調べるだけ
    DateRange(f, t)
        .filter(|&d| d.day() == 1 && d.weekday() == Weekday::Sun)
        .count() as u64
}

struct DateRange(NaiveDate, NaiveDate);

impl Iterator for DateRange {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let ret = self.0;
            self.0 = self.0.succ();
            Some(ret)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daterange_next() {
        let from = NaiveDate::from_ymd(2019, 1, 1);
        let to = NaiveDate::from_ymd(2019, 1, 7);
        let mut range = DateRange(from, to);

        assert_eq!(Some(NaiveDate::from_ymd(2019, 1, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 1, 2)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 1, 3)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 1, 4)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 1, 5)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 1, 6)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 1, 7)), range.next());
        assert_eq!(None, range.next());
    }

    #[test]
    fn test_solve() {
        let from = NaiveDate::from_ymd(2019, 1, 1);
        let to = NaiveDate::from_ymd(2019, 12, 31);
        assert_eq!(2, solve(from, to));
    }
}
