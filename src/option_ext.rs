use std::{
    fs::File,
    io::{self, stdin, BufReader, Read},
    path::PathBuf,
};

pub trait OptionExt {
    fn try_into(self) -> io::Result<BufReader<Box<dyn Read>>>;
}

impl OptionExt for Option<PathBuf> {
    fn try_into(self) -> io::Result<BufReader<Box<dyn Read>>> {
        let reader: Box<dyn Read> = match self {
            Some(path) => Box::new(File::open(path)?),
            None => Box::new(stdin()),
        };
        Ok(BufReader::new(reader))
    }
}
