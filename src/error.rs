use std::io;
use std::num::TryFromIntError;

use thiserror::Error;

use io_error::IoError;

mod io_error;

#[allow(clippy::pub_enum_variant_names)]
#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Error parsing string into a numeric value: {}", 0)]
    FailedStringParse(String),
    #[error("Invalid `day` argument specified.  Please enter a number from 1 to 25.")]
    InvalidDayArgument,
    #[error("Invalid `part` argument specified.  Please enter a number from 1 to 2.")]
    InvalidPartArgument,
    #[error("IO Error")]
    IoError(#[from] IoError),
    #[error("Overflow")]
    Overflow,
    #[error("Invalid OpCode value encountered: {}", 0)]
    InvalidOpCode(u32),
    #[error("Attempted to access invalid input position: {}", 0)]
    InvalidInputPosition(usize),
    #[error("OpCode Input 1 parameter not found: {}", 0)]
    OpCodeInput1ParamNotFound(usize),
    #[error("OpCode Input 2 parameter not found: {}", 0)]
    OpCodeInput2ParamNotFound(usize),
    #[error("OpCode Output parameter not found: {}", 0)]
    OpCodeOutputParamNotFound(usize),
    #[error("Error converting `u32` to `usize`: {}", 0)]
    U32NotConvertibleToUsize(#[from] TryFromIntError),
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Self::IoError(io_error.into())
    }
}
