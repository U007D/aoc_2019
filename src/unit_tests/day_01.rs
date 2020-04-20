use super::*;

#[test]
fn day_01__part_1_with_valid_known_test_masses_return_correct_fuel_totals() {
    // given
    let inputs = vec![vec![12], vec![14], vec![1969], vec![100756]];
    let expected_results = [2_u64, 2, 654, 33583];

    let mut res = Vec::new();
    inputs.into_iter().for_each(|input| {
        // when
        res.push(day_01(input));
    });

    // then
    let mut iter = expected_results.iter();
    res.into_iter().for_each(|res| {
        assert_eq!(res.unwrap().0, *iter.next().unwrap());
    });
}

#[test]
fn day_01__low_mass_rocket_returns_0_fuel_total() {
    // given
    let input = vec![1];
    let expected_result = (0, 0);

    // when
    let res = day_01(input);

    // then
    assert_eq!(res.unwrap(), expected_result);
}

#[test]
fn day_01__part_2_with_valid_known_test_masses_return_correct_fuel_totals() {
    // given
    let inputs = vec![vec![14], vec![1969], vec![100756]];
    let expected_results = [2_u64, 966, 50346];

    let mut res = Vec::new();
    inputs.into_iter().for_each(|input| {
        // when
        res.push(day_01(input));
    });

    // then
    let mut iter = expected_results.iter();
    res.into_iter().for_each(|res| {
        assert_eq!(res.unwrap().1, *iter.next().unwrap());
    });
}
