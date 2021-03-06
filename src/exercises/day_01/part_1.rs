#![allow(
    non_snake_case,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used,
    clippy::wildcard_imports
)]

use std::io::{BufRead, BufReader, Read};

use crate::{Error, Result};

use super::*;

/// Calculate the fuel required to launch a given mass.
pub fn part_1(buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<u32> {
    buf_reader.lines().try_fold(0_u32, |acc, res_string| {
        // Attempt to read the next line of input and try to convert it to a numeric value
        let s = res_string?;
        let value = s.trim().parse().map_err(|_| Error::FailedStringParse(s))?;

        // Accumulate total required fuel mass
        acc.checked_add(fuel_for_rocket_mass(value))
            .ok_or_else(|| Error::Overflow)
    })
}
