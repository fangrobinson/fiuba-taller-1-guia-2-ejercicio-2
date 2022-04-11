//! Contains IO file operations
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use super::file_errors::CoulNotOpenFile;

/// Convenient function to use on file lines reading.
///
/// Used to map incorrect lines into desired CoulNotOpenFile return.
fn handle_line_reading(item: Result<String, io::Error>) -> Result<String, CoulNotOpenFile> {
    match item {
        Ok(v) => Ok(v),
        Err(_) => Err(CoulNotOpenFile),
    }
}

/// Function to read file lines.
///
/// Returns:
///
///  - Vec<String> with file lines.
///
/// Throws:
///
///  - CoulNotOpenFile if file does not exist.
///
///  - CoulNotOpenFile if line could not be parsed.
pub fn read_file_lines(filename: impl AsRef<Path>) -> Result<Vec<String>, CoulNotOpenFile> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => return Err(CoulNotOpenFile),
    };

    BufReader::new(file)
        .lines()
        .map(handle_line_reading)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_file_that_does_not_exist_should_error() {
        assert!(read_file_lines("./file_that_does_not_exist.txt").is_err());
    }

    #[test]
    fn reading_file_that_doest_exist_should_ok() {
        assert!(read_file_lines("../Cargo.lock").is_err());
    }
}
