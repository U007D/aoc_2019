use std::io::{BufRead, BufReader, Read};

use crate::{Error, Result};

use super::{fuel_for_fuel_mass, fuel_for_rocket_mass};

#[cfg(test)]
mod unit_tests;

/// Calculate the fuel required to launch a given mass plus the fuel required to launch that fuel,
/// plus the additional fuel required to launch *that* fuel, and so on.
pub fn part_2(buf_reader: BufReader<Box<dyn Read>>) -> Result<u32> {
    buf_reader.lines().try_fold(0_u32, |acc, res_string| {
        // Attempt to read the next line of input and try to convert it to a numeric value
        let s = res_string?;
        let value = s.trim().parse().map_err(|_| Error::FailedStringParse(s))?;

        // Accumulate total required fuel mass
        acc.checked_add(fuel_for_fuel_mass(fuel_for_rocket_mass(value))?)
            .ok_or_else(|| Error::Overflow)
    })
}
