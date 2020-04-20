use crate::{Error, Result};

use super::{fuel_for_mass, fuel_for_mass_and_fuel};

#[cfg(test)]
mod unit_tests;

/// Calculate the fuel required to launch a given mass plus the fuel required to launch that fuel,
/// plus the additional fuel required to launch *that* fuel, and so on.
pub fn part_2<I>(input: I) -> Result<u32>
where
    I: IntoIterator<Item = u32>,
{
    input.into_iter().try_fold(0_u32, |acc, value| {
        acc.checked_add(fuel_for_mass_and_fuel(fuel_for_mass(value))?)
            .ok_or_else(|| Error::Overflow)
    })
}
