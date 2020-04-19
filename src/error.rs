mod io_error;
use io_error::IoError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Invalid `day` argument specified.  Please enter a number between 1 and 25")]
    InvalidDayArgument,
    #[error("IO Error")]
    IoError(#[from] IoError),
    #[error("Error parsing string into a numeric value")]
    AttemptedToParseNonNumericString(#[from] std::num::ParseIntError),
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Error::IoError(io_error.into())
    }
}
