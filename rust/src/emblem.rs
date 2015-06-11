extern crate byteorder;

use self::byteorder::{ByteOrder, BigEndian};

// typedef struct {
//         u8 gamecode[4];
//         u8 company[2];
//         u8 reserved01;  /*** Always 0xff ***/
//         u8 banner_fmt;
//         u8 filename[CARD_FILENAMELEN];
//         u32 time;
//         u32 icon_addr;  /*** Offset to banner/icon data ***/
//         u16 icon_fmt;
//         u16 icon_speed;
//         u8 unknown1;    /*** Permission key ***/
//         u8 unknown2;    /*** Copy Counter ***/
//         u16 index;              /*** Start block of savegame in memory card (Ignore - and throw away) ***/
//         u16 filesize8;  /*** File size / 8192 ***/
//         u16 reserved02; /*** Always 0xffff ***/
//         u32 comment_addr;
// } __attribute__((__packed__)) GCI;

const FZGX: [u8; 4] = *b"GFZE";
const SEGA: [u8; 2] = *b"8P";

pub struct Emblem {
  pub gamecode:     [u8; 4],  // GFZE8P
  pub company:      [u8; 2],  // 8P
  pub reserved01:    u8,      // 0xFF
  pub banner_fmt:    u8,      // 0x02
  pub filename:     [u8; 32],
  pub timestamp:    [u8; 4],
  pub something1:   [u8; 9],  // 0x00 0x00 0x00 0x60 0x00 0x02 0x00 0x03 0x04
  pub copy_counter:  u8,
  pub index:        [u8; 2],
  pub filesize8:    [u8; 2], // 0x00 0x03
  pub reserved02:   [u8; 2], // 0xFF 0xFF
  pub something2:   [u8; 4], // 0x00 0x00 0x00 0x04
  pub checksum:     [u8; 2],
  pub something3:   [u8; 2], // 0x04 0x01
  pub game_title:   [u8; 32], // "F-ZERO GX" 0x00...
  pub file_comment: [u8; 60], // "YY/MM/DD HH:MM" 0x00...
  pub banner_data:  [u8; 6144], // banner pixel data (92 x 32 px)
  pub icon_data:    [u8; 2048], // icon pixel data (64 x 64 px)
  pub emblem_data:  [u8; 8192], // emblem pixel data (64 x 64 px)
  pub padding:      [u8; 8095] // 0x00 padding
}

impl Default for Emblem {
    fn default() -> Self {
        Emblem {
          gamecode:      FZGX,
          company:       SEGA,
          reserved01:    0xFF,
          banner_fmt:    0x02,
          filename:     [0x00; 32],
          timestamp:    [0x00; 4],
          something1:   [0x00, 0x00, 0x00, 0x60, 0x00, 0x02, 0x00, 0x03, 0x04],
          copy_counter:  0x00,
          index:        [0x00; 2],
          filesize8:    [0x00; 2], // 0x00 0x03
          reserved02:   [0x00; 2], // 0xFF 0xFF
          something2:   [0x00; 4], // 0x00 0x00 0x00 0x04
          checksum:     [0x00; 2],
          something3:   [0x00; 2], // 0x04 0x01
          game_title:   [0x00; 32], // "F-ZERO GX" 0x00...
          file_comment: [0x00; 60], // "YY/MM/DD HH:MM" 0x00...
          banner_data:  [0x00; 6144], // banner pixel data (92 x 32 px)
          icon_data:    [0x00; 2048], // icon pixel data (64 x 64 px)
          emblem_data:  [0x00; 8192], // emblem pixel data (64 x 64 px)
          padding:      [0x00; 8095] // 0x00 padding
        }
    }
}

impl Emblem {
    pub fn set_filename(self: &mut Self, filename: String) {
    }

    pub fn set_timestamp(self: &mut Self, time: u32) {
        // let mut buf = [0x00; 4];
        // byteorder::BigEndian::write_u32(&mut buf, time);
        //
        // self.timestamp = buf;
    }
}
