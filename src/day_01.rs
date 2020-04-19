use crate::Result;
use std::io::{BufRead, BufReader, Read};

pub fn day_01(mut input: BufReader<Box<dyn Read>>) -> Result<u64> {
    let mut buf = String::new();
    let mut acc = 0;
    while input.read_line(&mut buf)? > 0 {
        acc += match buf.trim().parse::<u64>()? / 3 {
            val if val > 2 => val - 2,
            _ => 0,
        };
    }
    Ok(acc)
}
