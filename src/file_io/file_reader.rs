use std::{
    fs::File,
    io::{BufReader, BufRead, self},
    path::Path,
};

use super::file_errors::CoulNotOpenFile;

fn handle_line_reading(item: Result<String, io::Error>) -> Result<String, CoulNotOpenFile> {
    match item {
        Ok(v) => Ok(v),
        Err(_) => Err(CoulNotOpenFile),
    }
}

pub fn read_file_lines(filename: impl AsRef<Path>) -> Result<Vec<String>, CoulNotOpenFile> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => return Err(CoulNotOpenFile),
    };
    let mut output: Vec<String> = Vec::<String>::new();

    return BufReader::new(file).lines()
        .map(|l| handle_line_reading(l))
        .collect();
}
