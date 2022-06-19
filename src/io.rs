use std::{fs, io};

pub fn read_file(filepath: &str) -> io::Result<String> {
  fs::read_to_string(filepath)
}

pub fn write_file(filepath: &str, contents: String) -> io::Result<()> {
  fs::write(filepath, contents)
}
