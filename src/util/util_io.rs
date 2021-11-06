use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn read_text_file_internal(path: &Path) -> io::Result<String> {
    let mut content = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn write_text_file_internal(path: &Path, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    write!(&mut file, "{}", content)?;
    Ok(())
}

// Maps IO error to human-readable error
pub fn map_io_error<T>(e: io::Result<T>) -> Result<T, String> {
    e.map_err(|e| e.to_string())
}

// Convenience method to read from a text file
pub fn read_text_file(path: &Path) -> Result<String, String> {
    map_io_error(read_text_file_internal(path))
}

// Convenience method to write to a text file
pub fn write_text_file(path: &Path, content: &str) -> Result<(), String> {
    map_io_error(write_text_file_internal(path, content))
}
