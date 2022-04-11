use std::env;
use std::fmt;

mod file_io;
use file_io::file_reader as files;

mod lcs;
use lcs::diff;

/// Error to throw when diff could not run.
///
/// To be honest this would not be necessary
/// as it would be desirable to return more detailed errors
/// instead of this generic one.
///
/// Added this since could not make main -> Result<(), E>
/// use more that one Error type.
#[derive(Debug, Clone)]
pub struct CouldNotExecuteDiff;

impl fmt::Display for CouldNotExecuteDiff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not execute diff program")
    }
}

/// Main function of exercise 2 - Diff solution.
/// <p>
/// Prints diff of two files.
/// <p>
/// Path to files are received as CLI parameters.
/// <p>
/// Throws:
///
///  - CouldNotExecuteDiff if params len is not 3.
///
///  - CouldNotExecuteDiff if could not parse any of the two files.
///
///  - CouldNotExecuteDiff if could not initialize LCD grid (e.g. wrong index access).
fn main() -> Result<(), CouldNotExecuteDiff> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(CouldNotExecuteDiff);
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
