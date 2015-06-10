extern crate image2emblem;
extern crate time;
extern crate chrono;

use std::fs::File;
use std::path::Path;
use image::GenericImage;
use chrono::*;
use std::io::prelude::*;


fn python_total_seconds(microseconds: i64) -> f64 {
    microseconds as f64 / 10i64.pow(6) as f64
}

fn seconds_since_2000(now: chrono::datetime::DateTime<UTC>) -> f64 {
    let year_2000 = chrono::UTC.ymd(2000, 1, 1).and_hms(0, 0, 0);
    let duration = now - year_2000;
    let microseconds = duration.num_microseconds().unwrap();

    python_total_seconds(microseconds)
}

fn icon() -> [u8; 2048] {
    let icon_path = Path::new("../common/emblem_icon");
    let display = icon_path.display();
    let mut file = match File::open(&icon_path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut v: Vec<u8> = Vec::new();
    match file.read_to_end(&mut v) {
        Err(why) => panic!("couldn't read: {}", why),
        Ok(_) => {
            let mut arr: [u8; 2048] = [0x00; 2048];
            for index in 0..v.len() {
                arr[index] = v[index];
            }

            arr
        }
    }
}

fn main() {
}
