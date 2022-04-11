use std::env;
use std::fmt;

mod file_io;
use file_io::file_reader as files;

mod lcs;
use lcs::diff;

#[derive(Debug, Clone)]
pub struct CouldNotExecuteDiff;

impl fmt::Display for CouldNotExecuteDiff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not execute diff program")
    }
}

fn main() -> Result<(), CouldNotExecuteDiff> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("cantitdad incorrecta de argumentos");
    }
    let file_name_1 = &args[1];
    let file_name_2 = &args[2];
    let lines_file_1 = match files::read_file_lines(file_name_1) {
        Ok(v) => v,
        Err(_) => return Err(CouldNotExecuteDiff),
    };
    let lines_file_2 = match files::read_file_lines(file_name_2) {
        Ok(v) => v,
        Err(_) => return Err(CouldNotExecuteDiff),
    };

    let grid = match lcs::grid::LcsGrid::from_vecs_of_strings(&lines_file_1, &lines_file_2) {
        Ok(g) => g,
        Err(_) => return Err(CouldNotExecuteDiff),
    };

    diff::print_diff(
        &grid,
        &lines_file_1,
        &lines_file_2,
        lines_file_1.len(),
        lines_file_2.len(),
    );

    Ok(())
}
