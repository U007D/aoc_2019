use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    /// The AoC day (1-25).
    pub day: usize,
    /// The AoC day's question part (1-2).
    pub part: usize,
    /// Optional path to input file.  If not provided, data will be read from stdin.
    pub input_data_file: Option<PathBuf>,
}
