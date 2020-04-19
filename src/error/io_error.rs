use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("`io::Error`: {}", 0)]
pub struct IoError(#[from] io::Error);

impl PartialEq for IoError {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.kind() == rhs.0.kind()
    }
}
