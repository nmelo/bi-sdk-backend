use std::error;
use std::fmt;

#[derive(Debug)]
pub struct GenericError(pub String);

impl error::Error for GenericError {}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
