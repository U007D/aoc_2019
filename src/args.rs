use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    /// The AoC day (1-25).
    pub day: usize,
    /// Optional path to input file.  If not provided, data will be read from stdin.
    pub input_data_file: Option<PathBuf>,
}
