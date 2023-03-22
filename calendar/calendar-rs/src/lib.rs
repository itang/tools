#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! calendar lib.
//!
//!
use std::cmp::min;

use chrono::{offset, Datelike, Days, Weekday};
use tabled::{Style, Tabled};

//TODO: 月日历
//TODO: 周日历
//TODO: 年日历

const MAX_DAYS: u64 = 10_000;

/// pretty display days.
/// @param days: the days to display
pub fn display_day(days: u64) -> String {
    let now = offset::Local::now();

    let days = min(days, MAX_DAYS);
    let lines: Vec<DayItem> = (0..=days)
        .enumerate()
        .map(|(index, i)| {
            let to = now.checked_add_days(Days::new(i)).unwrap();
            let df: &str = &to.format("%Y-%m-%d").to_string();
            let wf = to.weekday().format().unwrap_or("");
            DayItem {
                no: index,
                date_f: df.to_string(),
                week_f: wf.to_string(),
            }
        })
        .collect();

    let mut table = tabled::Table::new(lines);
    table.with(Style::psql()).to_string()
}

trait WS {
    type Output;
    fn format(&self) -> Self::Output;
}

impl WS for Weekday {
    type Output = Option<&'static str>;
    fn format(&self) -> Self::Output {
        match self.num_days_from_monday() {
            0 => Some("星期一"),
            1 => Some("星期二"),
            2 => Some("星期三"),
            3 => Some("星期四"),
            4 => Some("星期五"),
            5 => Some("星期六"),
            6 => Some("星期日"),
            _ => None,
        }
    }
}

/// Day Row for table.
#[derive(Debug, Tabled)]
pub struct DayItem {
    /// The no.
    pub no: usize,
    /// The date.
    pub date_f: String,
    /// The href.
    pub week_f: String,
}
