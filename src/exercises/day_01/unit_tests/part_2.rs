use super::*;
use crate::day_01::part_2;

#[test]
fn part_1__part_2_with_valid_known_test_masses_return_correct_fuel_totals() {
    // given
    let readers = strs_to_readers(vec!["14", "1969", "100756"]);
    let expected_res = &[Ok(2_u32), Ok(966), Ok(50346)];

    // when
    // Pass each `Read`er in the collection to `part_2()` and record each return value
    let res = readers.into_iter().fold(Vec::new(), |mut acc, reader| {
        acc.push(part_2(BufReader::new(Box::new(reader.as_slice()))));
        acc
    });

    // then
    assert!(
        res.eq(expected_res),
        "{}",
        format!("res: {:?}, expected_res: {:?}", res, expected_res)
    );
}
