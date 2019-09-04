//! [Problem 19](https://projecteuler.net/problem=19)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2019))

use chrono::prelude::*;

pub struct Solver;

const FROM: (i32, u32) = (1901, 1);
const TO: (i32, u32) = (2000, 12);

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(
            NaiveDate::from_ymd(FROM.0, FROM.1, 1),
            NaiveDate::from_ymd(TO.0, TO.1, 1),
        )
    }
}

fn solve(f: NaiveDate, t: NaiveDate) -> u64 {
    // 1つずつ月をずらしながら調べるだけ
    MonthRange::new(f)
        .take_while(|&d| d <= t)
        .filter(|&d| d.day() == 1 && d.weekday() == Weekday::Sun)
        .count() as u64
}

struct MonthRange {
    ym: (i32, u32),
}

impl MonthRange {
    pub fn new(from: NaiveDate) -> MonthRange {
        MonthRange {
            ym: (from.year(), from.month()),
        }
    }
}

impl Iterator for MonthRange {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = NaiveDate::from_ymd(self.ym.0, self.ym.1, 1);

        self.ym = if ret.month() == 12 {
            (ret.year() + 1, 1)
        } else {
            (ret.year(), ret.month() + 1)
        };

        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_month_range_new() {
        let from = NaiveDate::from_ymd(2019, 1, 1);

        let range = MonthRange::new(from);
        assert_eq!((from.year(), from.month()), range.ym);
    }

    #[test]
    fn test_month_range_next() {
        let mut range = MonthRange {
            ym: (2019, 1),
        };

        assert_eq!(Some(NaiveDate::from_ymd(2019, 1, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 2, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 3, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 4, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 5, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 6, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 7, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 8, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 9, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 10, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 11, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2019, 12, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2020, 1, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2020, 2, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2020, 3, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2020, 4, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2020, 5, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2020, 6, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2020, 7, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2020, 8, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2020, 9, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2020, 10, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2020, 11, 1)), range.next());
        assert_eq!(Some(NaiveDate::from_ymd(2020, 12, 1)), range.next());
    }

    #[test]
    fn test_solve() {
        let from = NaiveDate::from_ymd(2019, 1, 1);
        let to = NaiveDate::from_ymd(2019, 12, 31);
        assert_eq!(2, solve(from, to));
    }
}
