use super::*;

#[test]
fn aoc__with_invalid_day_returns_error() {
    // given
    let day = 0;
    let input: BufReader<Box<dyn Read>> = BufReader::new(Box::new(std::io::empty()));
    let expected_result = Error::InvalidDayArgument;

    // when
    let res = aoc(day, input);

    // then
    assert_eq!(res.unwrap_err(), expected_result);
}

#[test]
fn aoc__with_valid_day_returns_ok() {
    // given
    let day = 1;
    let input: BufReader<Box<dyn Read>> = BufReader::new(Box::new(std::io::empty()));
    let expected_result = (0, 0);

    // when
    let res = aoc(day, input);

    // then
    assert_eq!(res.unwrap(), expected_result);
}

#[test]
fn aoc__with_invalid_input_returns_error() {
    // given
    let day = 1;
    let input: BufReader<Box<dyn Read>> = BufReader::new(Box::new(&b"12,4,30.aj9$28"[..]));

    // when
    let res = aoc(day, input);

    // then
    assert!(match res {
        Err(Error::AttemptedToParseNonNumericString(_)) => true,
        _ => false,
    });
}
