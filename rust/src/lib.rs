pub mod checksum;
pub mod emblem;

pub fn short_name(seconds: f64) -> String {
  let multiplier: f64 = 40500000f64;
  let tick: u64 = (seconds * multiplier) as u64;

  format!("fze0200002000{:14X}.dat", tick as u64)
}

pub fn full_name(filename: &str) -> String {
  format!("8P-GFZE-{}.gci", filename)
}
