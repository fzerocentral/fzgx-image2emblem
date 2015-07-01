extern crate image2emblem;
extern crate image;
extern crate chrono;
extern crate byteorder;


use std::fs::File;
use std::path::Path;
use chrono::*;
use std::io::prelude::*;
use std::process::exit;


fn python_total_seconds(microseconds: i64) -> f64 {
    microseconds as f64 / 10i64.pow(6) as f64
}

fn seconds_since_2000(now: chrono::datetime::DateTime<UTC>) -> f64 {
    let year_2000 = chrono::UTC.ymd(2000, 1, 1).and_hms(0, 0, 0);
    let duration = now - year_2000;

    match duration.num_microseconds() {
        Some(ms) => python_total_seconds(ms),
        None => panic!("No microseconds!")
    }
}

fn icon() -> [u8; 2048] {
    let icon_path = Path::new("../../common/emblem_icon");
    let display = icon_path.display();
    let mut file = match File::open(&icon_path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open '{}': {}", display, why),
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
    let path = Path::new("../../common/block.png");
    let mut img = image::open(&path).unwrap();
    let now = chrono::UTC::now();
    let seconds_since_2000 = seconds_since_2000(now);
    let alpha_threshold: i8 = 1;
    let icon_bytes = icon();
    let mut emblem = image2emblem::emblem::Emblem::default();

    let short_name = image2emblem::short_name(seconds_since_2000);
    let full_name = image2emblem::full_name(&short_name);

    let img64 = img.crop(0, 0, 64, 64);
    let img32 = img64.resize(32, 32, image::FilterType::Lanczos3);

    emblem.set_filename(short_name);
    emblem.set_timestamp(seconds_since_2000 as u32);
    let comment = format!("{} (Created using Rust awesomeness)", now.format("%y/%m/%d %H:%M"));

    emblem.set_comment(comment);
    emblem.set_emblem_data(img64, alpha_threshold);
    emblem.set_banner_data(img32, alpha_threshold);
    emblem.set_icon_data(icon_bytes);
    emblem.set_checksum();

    let mut emblem_file = File::create(full_name).unwrap();

    let result = emblem_file.write_all(&emblem.as_bytes());

    match result {
        Ok(_) => exit(0),
        Err(err) => panic!("Was not possible to write to file: {}", err)
    }
}
