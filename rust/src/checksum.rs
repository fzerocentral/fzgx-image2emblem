extern crate byteorder;

use self::byteorder::{ByteOrder, BigEndian};

pub fn checksum(bytes: &[u8]) -> Vec<u8> {
  let mut checksum = 0xFFFF;
  let generator_polynomial = 0x8408;

  for byte in bytes {
    checksum = checksum ^ byte;

    for i in (0..8) {
      if checksum & 1 == 1 {
        checksum = (checksum >> 1) ^ generator_polynomial
      } else {
        checksum = checksum >> 1
      }
    }
  }

  checksum = checksum ^ 0xFFFF;
  let mut buf = [0u8; 2];
  byteorder::BigEndian::write_u16(&mut buf, checksum as u16);

  return buf.to_vec();
}
