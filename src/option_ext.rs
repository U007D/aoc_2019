use std::{
    fs::File,
    io::{self, stdin, BufReader, Read},
    path::PathBuf,
};

pub trait OptionExt {
    fn try_into(self) -> io::Result<BufReader<Box<dyn Read>>>;
}

/// To make the top-level code more readable, provide a conversion function to transform an
/// Option<`PathBuf`> into a `File` reader (if `Some`) or `Stdin` reader (if `None`).  Because
/// this crate defines neither `Option<PathBuf>` nor the `From` trait, Rust orphan rules prevent us
/// from implementing `From<Option<PathBuf>> for Bufreader<_>`.  Thus, we have used an extension
/// trait to achieve a similar result.
impl OptionExt for Option<PathBuf> {
    fn try_into(self) -> io::Result<BufReader<Box<dyn Read>>> {
        let reader: Box<dyn Read> = match self {
            Some(path) => Box::new(File::open(path)?),
            None => Box::new(stdin()),
        };
        Ok(BufReader::new(reader))
    }
}
