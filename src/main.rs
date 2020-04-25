#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![forbid(bare_trait_objects)]
#![allow(clippy::match_bool)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

use structopt::StructOpt;

use aoc_2019::{Args, Result};
use option_ext::OptionExt;

mod option_ext;

fn main() -> Result<()> {
    let args = Args::from_args();
    let res = aoc_2019::aoc(args.day, args.part, args.input_data_file.try_into()?)?;
    println!("Day {}-part {} result: {:?}", args.day, args.part, res);

    Ok(())
}
