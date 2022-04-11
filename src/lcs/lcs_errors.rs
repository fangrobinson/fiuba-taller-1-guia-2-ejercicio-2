//! Contains LCS Grid related errors.
use std::fmt;

/// Error to throw when non valid access is attempted (e.g. access to cell outside indexes).
#[derive(Debug, Clone)]
pub struct LcsValueNotFound;

impl fmt::Display for LcsValueNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not found value")
    }
}

/// Error to throw when LCS Grid could not be created (e.g. LcsValueNotFound error was reached).
#[derive(Debug, Clone)]
pub struct CouldNotCreateLcsGrid;

impl fmt::Display for CouldNotCreateLcsGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not create lcs grid")
    }
}
