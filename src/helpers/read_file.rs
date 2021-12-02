use std::{fs::File, io::BufReader, path::Path};

pub fn lines_from_file(filename: impl AsRef<Path>) -> std::io::BufReader<File> {
  let file = File::open(filename).expect("no such file");
  return BufReader::new(file);
}
