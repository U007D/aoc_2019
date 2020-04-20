use std::io;

use thiserror::Error;

use io_error::IoError;

mod io_error;

#[allow(clippy::pub_enum_variant_names)]
#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Error parsing string into a numeric value")]
    AttemptedToParseNonNumericString(#[from] std::num::ParseIntError),
    #[error("Invalid `day` argument specified.  Please enter a number from 1 to 25.")]
    InvalidDayArgument,
    #[error("Invalid `part` argument specified.  Please enter a number from 1 to 2.")]
    InvalidPartArgument,
    #[error("IO Error")]
    IoError(#[from] IoError),
    #[error("Overflow")]
    Overflow,
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Self::IoError(io_error.into())
    }
}
