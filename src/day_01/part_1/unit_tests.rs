#![allow(non_snake_case, clippy::option_unwrap_used, clippy::result_unwrap_used)]

use super::*;

#[test]
fn part_1__with_valid_masses_returns_expected_fuel_totals() {
    // given
    let inputs = vec![vec![12], vec![14], vec![1969], vec![100_756]];
    let expected_results = [2_u32, 2, 654, 33583];

    let mut res = Vec::new();
    inputs.into_iter().for_each(|input| {
        // when
        res.push(part_1(input));
    });

    // then
    let mut iter = expected_results.iter();
    res.into_iter().for_each(|res| {
        assert_eq!(res, Ok(*iter.next().unwrap()));
    });
}

#[test]
fn part_1__low_mass_rocket_returns_0_and_not_negative_fuel_total() {
    // given
    let input = vec![1];
    let expected_result = 0;

    // when
    let res = part_1(input);

    // then
    assert_eq!(res.unwrap(), expected_result);
}
