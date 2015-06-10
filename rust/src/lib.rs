pub mod checksum;
pub mod emblem;

pub fn short_name(filename: Option<&str>, seconds: f64) -> Result<String, &'static str> {
  let multiplier: f64 = 40500000f64;
  let tick: u64 = (seconds * multiplier) as u64;

  match filename {
    Some(name) => {
      if name.len() < 18 {
        Ok(format!("fze1-{}.dat", name))
      } else {
        Err("emblem-filename should be 18 characters or less.")
      }
    },
    None => Ok(format!("fze0200002000{:14X}.dat", tick as u64))
  }
}

pub fn full_name(filename: &str) -> String {
  format!("8P-GFZE-{}.gci", filename)
}
