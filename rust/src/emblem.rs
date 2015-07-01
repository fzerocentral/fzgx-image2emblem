extern crate byteorder;
extern crate image;
extern crate itertools;

use self::byteorder::{ByteOrder, BigEndian};
use self::image::GenericImage;
use self::itertools::Itertools;

use std::io::prelude::*;
use std::fs::File;
use checksum::checksum;


fn gametitle() -> [u8; 32] {
    let mut gametitle: [u8; 32] = [0x00; 32];
    let fzgx = "F-Zero GX".as_bytes();

    for i in 0..fzgx.len() {
        gametitle[i] = fzgx[i];
    }

    gametitle
}

fn read_block(emblem_data: &mut Vec<u8>, image: &image::DynamicImage,
  alpha_threshold: i8, i: u32, j: u32) {
    let i = i as u32;
    let j = j as u32;

    for x in i..i+4 {
        for y in j..j+4 {
            let pixel = image.get_pixel(x, y).data;
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let a = pixel[3];

            match a < alpha_threshold as u8 {
                true => { emblem_data.push(0x00); },
                false => {
                    let red   = (r / 8) as u16;
                    let green = (g / 8) as u16;
                    let blue  = (b / 8) as u16;
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



const FZGX: [u8; 4] = *b"GFZE";
const SEGA: [u8; 2] = *b"8P";


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

  pub checksum:     [u8; 2],
  something3:   [u8; 2], // 0x04 0x01
  game_title:   [u8; 32], // "F-ZERO GX" 0x00...
  file_comment: [u8; 60], // "YY/MM/DD HH:MM" 0x00...
  pub banner_data:  [u8; 6144], // banner pixel data (92 x 32 px)
  icon_data:    [u8; 2048], // icon pixel data (64 x 64 px)
  emblem_data:  [u8; 8192], // emblem pixel data (64 x 64 px)
  padding:      [u8; 8096] // 0x00 padding
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
          padding:      [0x00; 8096] // 0x00 padding
        }
    }
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

    pub fn set_icon_data(self: &mut Self, icon: [u8; 2048]) {
        for i in 0..icon.len() {
            self.icon_data[i] = icon[i];
        }
    }

    pub fn set_banner_data(self: &mut Self, image: image::DynamicImage, alpha_threshold: i8) {
        let mut v = Vec::new();
        let mut banner_file = File::open("../../common/emblem_banner_base").unwrap();

        for block_row in (0..32).step(4) {
            let mut banner_data: [u8; 0x200] = [0x00; 0x200];
            match banner_file.read(&mut banner_data) {
                Err(err) => panic!("Could not read banner file: {}", err),
                _        => { ; }
            };

            for byte in banner_data.iter() {
                v.push(*byte);
            }

            for block_col in (0..32).step(4) {
                read_block(&mut v, &image, alpha_threshold, block_row, block_col);
            }
        }

        for i in 0..v.len() {
            self.banner_data[i] = v[i];
        }
    }

    pub fn set_checksum(self: &mut Self) {
        let mut v = Vec::new();

        for byte in self.something3.iter() {
            v.push(byte);
        }
        for byte in self.game_title.iter() {
            v.push(byte);
        }
        for byte in self.file_comment.iter() {
            v.push(byte);
        }
        for byte in self.banner_data.iter() {
            v.push(byte);
        }
        for byte in self.icon_data.iter() {
            v.push(byte);
        }
        for byte in self.emblem_data.iter() {
            v.push(byte);
        }
        for byte in self.padding.iter() {
            v.push(byte);
        }

        self.checksum = checksum(v);
    }

    pub fn as_bytes(self: Self) -> [u8; 24640] {
        let mut v: [u8; 24640] = [0x00; 24640];
        let mut index = 0;

        for byte in self.gamecode.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.company.iter() {
            v[index] = *byte;
            index += 1;
        }

        v[index] = self.reserved01;
        index += 1;
        v[index] = self.banner_fmt;
        index += 1;

        for byte in self.filename.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.timestamp.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.icon_addr.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.icon_fmt.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.icon_speed.iter() {
            v[index] = *byte;
            index += 1;
        }

        v[index] = self.permission;
        index += 1;
        v[index] = self.copy_counter;
        index += 1;

        for byte in self.index.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.filesize8.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.reserved02.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.comment_addr.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.checksum.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.something3.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.game_title.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.file_comment.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.banner_data.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.icon_data.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.emblem_data.iter() {
            v[index] = *byte;
            index += 1;
        }
        for byte in self.padding.iter() {
            v[index] = *byte;
            index += 1;
        }

        v
    }
}
