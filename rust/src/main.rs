extern crate image2emblem;
extern crate image;
extern crate chrono;
extern crate byteorder;


use std::fs::File;
use std::path::Path;
use image::GenericImage;
use chrono::*;
use std::io::prelude::*;
use byteorder::{ByteOrder, BigEndian};


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

fn setup_header_bytes(emblem: &mut image2emblem::emblem::Emblem, short_name: String, seconds: f64) {
    // # Short filename followed by 0 padding until 32 bytes
    // header_bytes += bytearray(emblem_short_filename)
    // header_bytes += bytearray(32 - len(emblem_short_filename))
    //
    // emblem.set_filename(short_name);
    emblem.set_timestamp(seconds_since_2000 as u32);

    //
    // # Constant bytes
    // header_bytes += bytearray([0, 0, 0, 0x60, 0, 2, 0, 3, 4])
    //
    // # Start block (2 bytes)
    // #
    // # TODO: Check if there is a better value to use here besides 0.
    // # Want to avoid the following error when we try to delete the file from a
    // # memcard in Dolphin: "Order of files in the File Directory do not match
    // # the block order[.] Right click and export all of the saves, and import
    // # the saves to a new memcard"
    // header_bytes += bytearray(struct.pack(">H", 0))
    // # Constant bytes
    // header_bytes += bytearray([0, 3, 0xFF, 0xFF, 0, 0, 0, 4])
    //
    // return header_bytes
}

// fn setup_more_info_bytes(now: DateTime<UTC>) {
    // more_info_bytes = bytearray()
    // # Constant bytes
    // more_info_bytes += bytearray([4, 1])
    // # Game title followed by 0 padding until 32 bytes
    // more_info_bytes += bytearray("F-ZERO GX")
    // more_info_bytes += bytearray(32 - len("F-ZERO GX"))
    // # File comment followed by 0 padding until 60 bytes
    // comment_str = now.strftime("%y/%m/%d %H:%M")
    //
    // if additional_comment:
    //     comment_str += " (Created using third party code)"
    //
    // more_info_bytes += bytearray(comment_str)
    // more_info_bytes += bytearray(60 - len(comment_str))
    //
    // return more_info_bytes
// }

fn main() {
    let path = Path::new("../../processing/source/ImageToEmblem1_01/data/resources/fzcLogo.png");
    let mut img = image::open(&path).unwrap();
    let now = chrono::UTC::now();
    let seconds_since_2000 = seconds_since_2000(now);
    let alpha_threshold = 1;
    let icon_bytes = icon();
    let mut emblem = image2emblem::emblem::Emblem::default();

    let short_name = image2emblem::short_name(None, seconds_since_2000);
    let full_name = image2emblem::full_name(&short_name);

    let img64 = img.crop(0, 0, 64, 64);
    let img32 = img64.resize(32, 32, image::FilterType::Lanczos3);

    setup_header_bytes(&mut emblem, short_name, seconds_since_2000);

    // header_bytes = setup_header_bytes(emblem_short_filename, seconds_since_start_of_2000)
    // more_info_bytes = setup_more_info_bytes(now, args.additional_comment)
    //
    // # TODO: Test non-RGBA stuff going through crop or resize64.
    // # (That, or know when to tell the user to resize/convert themselves...)
    // img64 = edge_options(img, args.edge_option)
    //
    // # TODO: Check how the 64 to 32 resize is done by the game. Not a
    // # big deal though, it just means the banner may look slightly different
    // # than it should in a memcard manager.
    // img32 = img.resize((32,32), Image.LANCZOS)
    // img64_data = img64.getdata()
    //
    // emblem_pixel_bytes = emblem(img64_data, alpha_threshold)
    // banner_bytes = banner(img32, alpha_threshold)
    //
    // # A bunch of zeros until the end of 3 Gamecube memory blocks
    // end_padding_bytes = bytearray(0x6040 - 0x40A0)
    //
    // post_checksum_bytes = more_info_bytes + banner_bytes \
    //   + icon_bytes + emblem_pixel_bytes + end_padding_bytes
    //
    // checksum_bytes = checksum(post_checksum_bytes)
    //
    // emblem_file = open(emblem_full_filename, 'wb')
    // emblem_file.write(header_bytes + checksum_bytes + post_checksum_bytes)
    // emblem_file.close()
}
