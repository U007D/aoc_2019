use crate::{Input, Result};

pub fn day_01(input: Input) -> Result<(u64, u64)> {
    Ok((part_1(input.iter()), part_2(input.iter())))
}

fn part_1<'a, I: Iterator<Item = &'a u64>>(iter: I) -> u64 {
    iter.fold(0, |acc, value| acc + (value / 3).saturating_sub(2))
}

fn part_2<'a, I: Iterator<Item = &'a u64>>(iter: I) -> u64 {
    fn self_compensating_fuel_mass(mass: u64) -> u64 {
        match mass {
            0 => mass,
            _ => mass + self_compensating_fuel_mass((mass / 3).saturating_sub(2)),
        }
    }

    iter.fold(0, |acc, value| {
        acc + self_compensating_fuel_mass((value / 3).saturating_sub(2))
    })
}
