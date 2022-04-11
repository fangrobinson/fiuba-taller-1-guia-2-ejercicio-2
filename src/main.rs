use std::env;

mod file_io;
use file_io::file_reader as files;

mod lcs;

fn main() -> Result<(), file_io::file_errors::CoulNotOpenFile> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("cantitdad incorrecta de argumentos");
    }
    let file_name_1 = &args[1];
    let file_name_2 = &args[2];
    let lines_file_1 = match files::read_file_lines(file_name_1) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let lines_file_2 = match files::read_file_lines(file_name_2) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    return Ok(());
}
