use crate::{Error, Result};

use super::fuel_for_rocket_mass;

#[cfg(test)]
mod unit_tests;

/// Calculate the fuel required to launch a given mass.
pub fn part_1<I>(input: I) -> Result<u32>
where
    I: IntoIterator<Item = u32>,
{
    input
        .into_iter()
        .try_fold(0_u32, |acc, value| {
            acc.checked_add(fuel_for_rocket_mass(value))
        })
        .ok_or_else(|| Error::Overflow)
}
