use std::ops::Div;

pub use part_1::part_1;
pub use part_2::part_2;

use crate::{Error, Result};

mod part_1;
mod part_2;

// `.saturating_sub()` not yet stable as `const fn`
#[allow(clippy::missing_const_for_fn)]
#[inline]
fn fuel_for_mass(mass: u32) -> u32 {
    (mass.div(3)).saturating_sub(2)
}

#[inline]
fn fuel_for_mass_and_fuel(mass: u32) -> Result<u32> {
    match mass {
        0 => Ok(mass),
        _ => mass
            .checked_add(fuel_for_mass_and_fuel(fuel_for_mass(mass))?)
            .ok_or_else(|| Error::Overflow),
    }
}
