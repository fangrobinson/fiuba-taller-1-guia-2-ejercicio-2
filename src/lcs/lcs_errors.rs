use std::fmt;

#[derive(Debug, Clone)]
pub struct LcsValueNotFound;

impl fmt::Display for LcsValueNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not found value")
    }
}
