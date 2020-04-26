use super::*;
use crate::day_02::part_1;

#[test]
fn part_1__sample_program_1_yields_expected_output() {
    // Given
    let program = str_to_reader("1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50");
    let expected_res = Ok(vec![3500_usize, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);

    // When
    let res = part_1(program);

    // Then
    assert!(
        res.eq(expected_res),
        "{}",
        format!("res: {:?}, expected_res: {:?}", res, expected_res)
    );
}
