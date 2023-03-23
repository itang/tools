#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! calendar lib.
//!
//!
use std::cmp::min;

use chrono::{offset, Datelike, Days, Weekday};
use tabled::{Style, Tabled};
use tap::Tap;

//TODO: 月日历
//TODO: 周日历
//TODO: 年日历

const MAX_DAYS: u64 = 10_000;

/// pretty display days.
/// @param days: the days to display
pub fn display_day(days: u64) -> String {
    let now = offset::Local::now();

    let days = min(days, MAX_DAYS);

    let rows: Vec<DayItem> = (0..=days)
        .enumerate()
        .flat_map(|(index, i)| {
            let to = now
                .checked_add_days(Days::new(i))
                .expect("checked add days");
            let df: &str = &to.format("%Y-%m-%d").to_string();
            let wf = to.weekday().format().unwrap_or("");

            vec![DayItem::new(index.to_string(), df, wf)].tap_mut(|it| {
                if to.weekday().is_last_day_of_week() {
                    it.push(DayItem::blank());
                }
            })
        })
        .collect();

    tabled::Table::new(rows)
        .tap_mut(|it| {
            it.with(Style::psql());
        })
        .to_string()
}

trait WS {
    type Output;
    fn format(&self) -> Self::Output;

    fn is_last_day_of_week(&self) -> bool;
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

    fn is_last_day_of_week(&self) -> bool {
        self.num_days_from_monday() == 6
    }
}

/// Day Row for table.
#[derive(Debug, Tabled)]
pub struct DayItem {
    /// The no.
    pub no: String,
    /// The date.
    pub date_f: String,
    /// The href.
    pub week_f: String,
}

impl DayItem {
    /// Make a DayItem.
    pub fn new<A, B>(no: A, date_f: B, week_f: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            no: no.into(),
            date_f: date_f.into(),
            week_f: week_f.into(),
        }
    }
    /// Make a blank DayItem.
    pub fn blank() -> Self {
        Self::new("", "-", "-")
    }
}
