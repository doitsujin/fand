use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::{ self, Read, Write };

fn read_text_file_internal(path: &Path) -> io::Result<String> {
  let mut content = String::new();
  let mut file = try!(File::open(path));
  try!(file.read_to_string(&mut content));
  Ok(content)
}


fn write_text_file_internal(path: &Path, content: &str) -> io::Result<()> {
  let mut file = try!(File::create(path));
  try!(write!(&mut file, "{}", content));
  Ok(())
}

// Maps IO error to human-readable error
pub fn map_io_error<T>(e: io::Result<T>) -> Result<T, String> {
  e.map_err(|e| e.description().to_string())
}

// Convenience method to read from a text file
pub fn read_text_file(path: &Path) -> Result<String, String> {
  map_io_error(read_text_file_internal(path))
}

// Convenience method to write to a text file
pub fn write_text_file(path: &Path, content: &str) -> Result<(), String> {
  map_io_error(write_text_file_internal(path, content))
}
