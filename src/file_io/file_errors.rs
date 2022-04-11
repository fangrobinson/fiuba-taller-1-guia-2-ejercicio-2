use std::fmt;

#[derive(Debug, Clone)]
pub struct CoulNotOpenFile;

impl fmt::Display for CoulNotOpenFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not open file")
    }
}
