use std::io::{BufRead, BufReader, Read};

pub use args::Args;
use day_01::day_01;
pub use error::Error;

mod args;
pub mod consts;
mod day_01;
mod error;
#[cfg(test)]
mod unit_tests;

pub type Result<T, E = Error> = std::result::Result<T, E>;
type BufferedReader = BufReader<Box<dyn Read>>;
type Input = Vec<u64>;

pub fn aoc(day: usize, buf_reader: BufferedReader) -> Result<(u64, u64)> {
    let input = input(buf_reader)?;
    match day {
        1 => day_01(input),
        _ => Err(Error::InvalidDayArgument),
    }
}

fn input(mut buf_reader: BufferedReader) -> Result<Input> {
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
