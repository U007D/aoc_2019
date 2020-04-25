#![allow(non_snake_case, clippy::option_unwrap_used, clippy::result_unwrap_used)]

use super::*;

#[test]
fn part_1__part_2_with_valid_known_test_masses_return_correct_fuel_totals() {
    // given
    let inputs = vec![vec![14], vec![1969], vec![100_756]];
    let expected_results = [2_u32, 966, 50346];

    let mut res = Vec::new();
    inputs.into_iter().for_each(|input| {
        // when
        res.push(part_2(input));
    });

    // then
    let mut iter = expected_results.iter();
    res.into_iter().for_each(|res| {
        assert_eq!(res, Ok(*iter.next().unwrap()));
    });
}
