extern crate image2emblem;
extern crate time;
extern crate chrono;

use chrono::*;

fn python_total_seconds(microseconds: i64) -> f64 {
    microseconds as f64 / 10i64.pow(6) as f64
}

fn seconds_since_2000(now: chrono::datetime::DateTime<UTC>) -> f64 {
    let year_2000 = chrono::UTC.ymd(2000, 1, 1).and_hms(0, 0, 0);
    let duration = now - year_2000;
    let microseconds = duration.num_microseconds().unwrap();

    python_total_seconds(microseconds)
}

fn main() {
}
