extern crate byteorder;
extern crate image;
extern crate itertools;

use self::byteorder::{ByteOrder, BigEndian};
use self::image::GenericImage;
use self::itertools::Itertools;

// typedef struct {
//   00    u8 gamecode[4];
//   04    u8 company[2];
//   06    u8 reserved01;  /*** Always 0xff ***/
//   07    u8 banner_fmt;
//   08    u8 filename[CARD_FILENAMELEN];
//   40    u32 time;
//   44    u32 icon_addr;  /*** Offset to banner/icon data ***/
//   48    u16 icon_fmt;
//   50    u16 icon_speed;
//   52    u8 unknown1;    /*** Permission key ***/
//   53    u8 unknown2;    /*** Copy Counter ***/
//   54    u16 index;              /*** Start block of savegame in memory card (Ignore - and throw away) ***/
//   56    u16 filesize8;  /*** File size / 8192 ***/
//   58    u16 reserved02; /*** Always 0xffff ***/
//   60    u32 comment_addr;
// } __attribute__((__packed__)) GCI;

fn gametitle() -> [u8; 32] {
    let mut gametitle: [u8; 32] = [0x00; 32];
    let fzgx = "F-Zero GX".as_bytes();

    for i in 0..fzgx.len() {
        gametitle[i] = fzgx[i];
    }

    gametitle
}


const FZGX: [u8; 4] = *b"GFZE";
const SEGA: [u8; 2] = *b"8P";

pub struct GCI {
    gamecode:     [u8; 4],  // GFZE
    company:      [u8; 2],  // 8P
    reserved01:    u8,      // 0xFF
    banner_fmt:    u8,      // 0x02
    filename:     [u8; 32],
    timestamp:    [u8; 4],
    icon_addr:    [u8; 4], // 0x00 0x00 0x00 0x60
    icon_fmt:     [u8; 2], // 0x00 0x02
    icon_speed:   [u8; 2], // 0x00 0x03
    permission:    u8,
    copy_counter:  u8,
    index:        [u8; 2],
    filesize8:    [u8; 2], // 0x00 0x03
    reserved02:   [u8; 2], // 0xFF 0xFF
    comment_addr: [u8; 4], // 0x00 0x00 0x00 0x04
}

pub struct Emblem {
  gamecode:     [u8; 4],  // GFZE
  company:      [u8; 2],  // 8P
  reserved01:    u8,      // 0xFF
  banner_fmt:    u8,      // 0x02
  filename:     [u8; 32],
  timestamp:    [u8; 4],
  icon_addr:    [u8; 4], // 0x00 0x00 0x00 0x60
  icon_fmt:     [u8; 2], // 0x00 0x02
  icon_speed:   [u8; 2], // 0x00 0x03
  permission:    u8,
  copy_counter:  u8,
  index:        [u8; 2],
  filesize8:    [u8; 2], // 0x00 0x03
  reserved02:   [u8; 2], // 0xFF 0xFF
  comment_addr: [u8; 4], // 0x00 0x00 0x00 0x04

  checksum:     [u8; 2],
  something3:   [u8; 2], // 0x04 0x01
  game_title:   [u8; 32], // "F-ZERO GX" 0x00...
  file_comment: [u8; 60], // "YY/MM/DD HH:MM" 0x00...
  banner_data:  [u8; 6144], // banner pixel data (92 x 32 px)
  icon_data:    [u8; 2048], // icon pixel data (64 x 64 px)
  emblem_data:  [u8; 8192], // emblem pixel data (64 x 64 px)
  padding:      [u8; 8095] // 0x00 padding
}

impl Default for Emblem {
    fn default() -> Self {
        let game_title = gametitle();

        Emblem {
          gamecode:      FZGX,
          company:       SEGA,
          reserved01:    0xFF,
          banner_fmt:    0x02,
          filename:     [0x00; 32],
          timestamp:    [0x00; 4],
          icon_addr:    [0x00, 0x00, 0x00, 0x60],
          icon_fmt:     [0x00, 0x02],
          icon_speed:   [0x00, 0x03],
          permission:    0x04,
          copy_counter:  0x00,
          index:        [0x00, 0x00],
          filesize8:    [0x00, 0x03],
          reserved02:   [0xFF, 0xFF],
          comment_addr: [0x00, 0x00, 0x00, 0x04],

          checksum:     [0x00; 2],
          something3:   [0x04, 0x01],
          game_title:   game_title, // "F-ZERO GX" 0x00...
          file_comment: [0x00; 60], // "YY/MM/DD HH:MM" 0x00...

          banner_data:  [0x00; 6144], // banner pixel data (92 x 32 px)
          icon_data:    [0x00; 2048], // icon pixel data (64 x 64 px)
          emblem_data:  [0x00; 8192], // emblem pixel data (64 x 64 px)
          padding:      [0x00; 8095] // 0x00 padding
        }
    }
}

fn read_block(emblem_data: &mut Vec<u8>, image: &image::DynamicImage,
  alpha_threshold: i8, i: u32, j: u32) {
    let i = i as u32;
    let j = j as u32;

    for x in i..i+4 {
        for y in j..j+4 {
            let pixel = image.get_pixel(i, j).data;
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let a = pixel[3];

            match a < alpha_threshold as u8 {
                true => { emblem_data.push(0x00); },
                false => {
                    let red   = ((r as f32) / 8.0).floor() as u16;
                    let green = ((r as f32) / 8.0).floor() as u16;
                    let blue  = ((r as f32) / 8.0).floor() as u16;
                    let alpha: u16 = 1;
                    let value: u16 = 32768*alpha + 1024*red + 32*green + blue;

                    let mut buf: [u8; 2] = [0x00; 2];
                    byteorder::BigEndian::write_u16(&mut buf, value as u16);

                    for byte in buf.iter() {
                        emblem_data.push(*byte);
                    }
                }
            }

            // print!("{} {} {} {} ", r, g, b, a);
        }

        // println!("");
    }
    // println!("");
}


impl Emblem {
    pub fn set_filename(self: &mut Self, filename: String) {
        let bytes = filename.as_bytes();

        for i in 0..bytes.len() {
            self.filename[i] = bytes[i];
        }
    }

    pub fn set_timestamp(self: &mut Self, time: u32) {
        let mut buf = [0x00; 4];
        byteorder::BigEndian::write_u32(&mut buf, time);

        self.timestamp = buf;
    }

    pub fn set_comment(self: &mut Self, comment: String) {
        let comment_bytes = comment.as_bytes();

        for i in 0..comment_bytes.len() {
            self.file_comment[i] = comment_bytes[i];
        }
    }

    pub fn set_emblem_data(self: &mut Self, image: image::DynamicImage, alpha_threshold: i8) {
        let mut v = Vec::new();

        for block_row in (0..image.width()).step(4) {
            for block_col in (0..image.width()).step(4) {
                read_block(&mut v, &image, alpha_threshold, block_row, block_col);
            }
        }

        for i in 0..v.len() {
            self.emblem_data[i] = v[i];
        }
    }

    pub fn set_banner_data(self: &mut Self, image: image::DynamicImage, alpha_threshold: i8) {
        let mut v = Vec::new();

        for block_row in (0..image.width()).step(4) {
            for block_col in (0..image.width()).step(4) {
                read_block(&mut v, &image, alpha_threshold, block_row, block_col);
            }
        }

        for i in 0..v.len() {
            self.banner_data[i] = v[i];
        }
    }
}
