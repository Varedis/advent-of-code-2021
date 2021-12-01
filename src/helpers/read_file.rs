use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<u32> {
  let file = File::open(filename).expect("no such file");
  let buf = BufReader::new(file);
  buf.lines()
      .map(|l| l.unwrap().parse::<u32>().unwrap())
      .collect()
}