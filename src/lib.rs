#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![allow(clippy::match_bool, clippy::missing_errors_doc)]
#![forbid(bare_trait_objects)]
#![forbid(unsafe_code)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
// Safety-critical application lints
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

use std::io::{BufRead, BufReader, Read};

pub use args::Args;
pub use error::Error;

mod args;
pub mod consts;
mod day_01;
mod error;
#[cfg(test)]
mod unit_tests;

pub type Result<T, E = Error> = std::result::Result<T, E>;
type BufferedReader = BufReader<Box<dyn Read>>;

/// Reading and parsing the input data simplifies the core problem implementations for each day.  In
/// the event the data were too large or it were otherwise impractical to pre-load and parse all the
/// input data, the strategy would be to refactor the code to read on a line-by-line basis.  
/// Proper error handling in this scenario is more complex as, for example, `BufRead::Lines` returns
/// `Option<Result<String>>`, which, when `u32::parse()`ed, yields values in the form of
/// `Result<Result<u32>>`.  Further investigation will undoubtedly provide simpler avenues, but for
/// now, pre-fetching the input data is correct, maintainable and an expedient way forward.
fn read_input(mut buf_reader: BufferedReader) -> Result<impl IntoIterator<Item = u32>> {
    let mut input = Vec::new();
    {
        let mut buf = String::new();
        while buf_reader.read_line(&mut buf)? > 0 {
            input.push(buf.trim().parse()?);
            buf.clear();
        }
    }
    Ok(input)
}

/// This is the main library entry-point.  This function preps the input data and dispatches the
/// data to the correct function based on `day` and `part` for processing.
pub fn aoc(day: usize, part: usize, buf_reader: BufferedReader) -> Result<u32> {
    let input = read_input(buf_reader)?;
    match day {
        1 => match part {
            1 => day_01::part_1(input),
            2 => day_01::part_2(input),
            _ => Err(Error::InvalidPartArgument),
        },
        _ => Err(Error::InvalidDayArgument),
    }
}
