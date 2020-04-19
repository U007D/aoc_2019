#![allow(non_snake_case)]
use super::*;

#[test]
fn aoc__with_invalid_day_returns_error() {
    // given aoc
    let day = 0;
    let buf_reader: BufReader<Box<dyn Read>> = BufReader::new(Box::new(std::io::empty()));

    // when
    let res = aoc(day, buf_reader);

    // then
    assert_eq!(res, Err(Error::InvalidDayArgument));
}
