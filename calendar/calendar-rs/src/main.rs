use std::cmp::min;

use chrono::{Datelike, Days, offset, Weekday};
use clap::Parser;

//TODO: 月日历
//TODO: 周日历
//TODO: 年日历

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8)]
    days: u64,
}

const MAX_DAYS: u64 = 10_000;

fn main() {
    let args = Args::parse();

    let now = offset::Local::now();

    let days = min(args.days, MAX_DAYS);
    let lines: Vec<String> = (0..=days).map(|i| {
        let to = now.checked_add_days(Days::new(i)).unwrap();
        let df: &str = &to.format("%Y-%m-%d").to_string();
        let wf = to.weekday().format().unwrap_or("");
        format!("{i}: {df} - {wf}")
    }).collect();
    let output = lines.join("\n");
    println!("{output}");
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
