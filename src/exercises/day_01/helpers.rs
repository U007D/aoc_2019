use std::ops::Div;

use crate::{Error, Result};

// `.saturating_sub()` not yet stable as `const fn`
#[allow(clippy::missing_const_for_fn)]
#[inline]
pub fn fuel_for_rocket_mass(mass: u32) -> u32 {
    (mass.div(3)).saturating_sub(2)
}

#[inline]
pub fn fuel_for_fuel_mass(mass: u32) -> Result<u32> {
    match mass {
        0 => Ok(mass),
        _ => mass
            .checked_add(fuel_for_fuel_mass(fuel_for_rocket_mass(mass))?)
            .ok_or_else(|| Error::Overflow),
    }
}
