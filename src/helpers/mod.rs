use std::{io::prelude::*, path::Path};

mod read_file;

pub fn format_file_lines_as_int(filename: impl AsRef<Path>) -> Vec<u32> {
  let buf = read_file::lines_from_file(filename);

  buf
    .lines()
    .map(|l| l.unwrap().parse::<u32>().unwrap())
    .collect()
}
