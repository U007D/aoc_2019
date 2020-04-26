use crate::day_01::part_1;

use super::*;

#[test]
fn part_1__with_valid_masses_returns_expected_fuel_totals() {
    // given
    #[allow(clippy::string_lit_as_bytes)]
    let readers = strs_to_readers(vec!["12", "14", "1969", "100756"]);
    let expected_res = &[Ok(2_u32), Ok(2), Ok(654), Ok(33583)];

    // when
    // Pass each `Read`er in the collection to `part_1()` and record each return value
    let res = readers.into_iter().fold(Vec::new(), |mut acc, buf_reader| {
        acc.push(part_1(BufReader::new(Box::new(buf_reader.as_slice()))));
        acc
    });

    // then
    assert!(
        res.eq(expected_res),
        "{}",
        format!("res: {:?}, expected_res: {:?}", res, expected_res)
    );
}

#[test]
fn part_1__low_mass_rocket_returns_0_and_not_negative_fuel_total() {
    // given
    let reader = str_to_reader("1");
    let expected_res = Ok(0);

    // when
    let res = part_1(BufReader::new(Box::new(reader.as_slice())));

    // then
    assert_eq!(res, expected_res);
}
