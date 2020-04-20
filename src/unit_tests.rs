#![allow(non_snake_case, clippy::option_unwrap_used, clippy::result_unwrap_used)]

use super::*;

#[test]
fn aoc__with_invalid_day_returns_error() {
    // given
    let day = 0;
    let part = 1;
    let input: BufReader<Box<dyn Read>> = BufReader::new(Box::new(std::io::empty()));
    let expected_result = Error::InvalidDayArgument;

    // when
    let res = aoc(day, part, input);

    // then
    assert_eq!(res.unwrap_err(), expected_result);
}

#[test]
fn aoc__with_invalid_part_returns_error() {
    // given
    let day = 1;
    let part = 3;
    let input: BufReader<Box<dyn Read>> = BufReader::new(Box::new(std::io::empty()));
    let expected_result = Error::InvalidPartArgument;

    // when
    let res = aoc(day, part, input);

    // then
    assert_eq!(res.unwrap_err(), expected_result);
}

#[test]
fn aoc__with_valid_day_returns_ok() {
    // given
    let day = 1;
    let part = 1;
    let input: BufReader<Box<dyn Read>> = BufReader::new(Box::new(std::io::empty()));
    let expected_result = Ok(0);

    // when
    let res = aoc(day, part, input);

    // then
    assert_eq!(res, expected_result);
}

#[test]
fn aoc__with_invalid_input_returns_error() {
    // given
    let day = 1;
    let part = 1;
    let input: BufReader<Box<dyn Read>> = BufReader::new(Box::new(&b"12,4,30.aj9$28"[..]));

    // when
    let res = aoc(day, part, input);

    // then
    assert!(match res {
        Err(Error::AttemptedToParseNonNumericString(_)) => true,
        _ => false,
    });
}

#[test]
fn aoc__changing_part_dispatches_to_distinct_functions() {
    // given
    let day = 1;
    let part_1 = 1;
    let part_2 = 2;
    let input_1: BufReader<Box<dyn Read>> = BufReader::new(Box::new(&b"100"[..]));
    let input_1_again: BufReader<Box<dyn Read>> = BufReader::new(Box::new(&b"100"[..]));
    let expected_result_1 = Ok(31);
    let expected_result_2 = Ok(39);

    // when
    let res_1 = aoc(day, part_1, input_1);
    let res_2 = aoc(day, part_2, input_1_again);

    // then
    assert_eq!(res_1, expected_result_1);
    assert_eq!(res_2, expected_result_2);
}
