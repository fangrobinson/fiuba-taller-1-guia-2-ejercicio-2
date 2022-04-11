//! Contains IO errors.
use std::fmt;

/// Error to throw when diff could not run (e.g. file does not exist).
#[derive(Debug, Clone)]
pub struct CoulNotOpenFile;

impl fmt::Display for CoulNotOpenFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not open file")
    }
}
