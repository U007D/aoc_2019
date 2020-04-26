#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
#![warn(
    clippy::all,
    clippy::nursery,
    missing_debug_implementations,
    clippy::pedantic,
    rust_2018_idioms
)]
#![allow(clippy::match_bool, clippy::missing_errors_doc)]
#![forbid(bare_trait_objects)]
#![forbid(unsafe_code)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
// Safety-critical application lints
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

use std::io::{BufReader, Read};

pub use aoc_result::AocReturn;
pub use args::Args;
pub use error::Error;
use exercises::day_01;

mod aoc_result;
mod args;
pub mod consts;
mod error;
mod exercises;
#[cfg(test)]
mod unit_tests;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// This is the main library entry-point.  This function preps the input data and dispatches the
/// data to the correct function based on `day` and `part` for processing, returning the result
/// to the caller.
pub fn aoc(day: usize, part: usize, buf_reader: BufReader<Box<dyn Read>>) -> Result<AocReturn> {
    match day {
        1 => match part {
            1 => day_01::part_1(buf_reader).map(|v| v.into()),
            2 => day_01::part_2(buf_reader).map(|v| v.into()),
            _ => Err(Error::InvalidPartArgument),
        },
        // 2 => match part {
        //     1 => day_02::part_1(buf_reader).map(|v| v.into()),
        //     //2 => day_02::part_2(buf_reader).map(|v| v.into()),
        //     _ => Err(Error::InvalidPartArgument),
        // },
        _ => Err(Error::InvalidDayArgument),
    }
}
