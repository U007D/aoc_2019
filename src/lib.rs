mod args;
pub mod consts;
mod day_01;
mod error;
#[cfg(test)]
mod unit_tests;

pub use args::Args;
use day_01::day_01;
pub use error::Error;
use std::io::{BufReader, Read};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn aoc(day: usize, buf_reader: BufReader<Box<dyn Read>>) -> Result<u64> {
    match day {
        1 => day_01(buf_reader),
        _ => Err(Error::InvalidDayArgument),
    }
}
