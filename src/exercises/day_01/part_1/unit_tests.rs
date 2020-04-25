#![allow(non_snake_case, clippy::option_unwrap_used, clippy::result_unwrap_used)]

use super::*;

#[test]
fn part_1__with_valid_masses_returns_expected_fuel_totals() {
    // given
    let inputs: Vec<BufReader<Box<dyn Read>>> = vec![&[12], &[14], &[1969], &[100_756]];
    let expected_results = &[Ok(2_u32), Ok(2), Ok(654), Ok(33583)];

    // when
    let res = inputs.fold(
        Vec::new(),
        |mut acc: Vec<Result<u32>>, buf_reader: BufReader<Box<dyn Read>>| {
            acc.push(part_1(buf_reader));
            acc
        },
    );

    // then
    res.eq(&expected_results);
}

#[test]
fn part_1__low_mass_rocket_returns_0_and_not_negative_fuel_total() {
    // given
    let buf_reader: BufReader<Box<dyn Read>> = BufReader::new(Box::new(vec![1].as_slice()));
    let expected_result = Ok(0);

    // when
    let res = part_1(buf_reader);

    // then
    assert_eq!(res, expected_result);
}
