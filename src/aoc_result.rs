#[allow(clippy::doc_markdown)]
/// Some AoC puzzles return a `u32`, while others return a `Vec<u32`.  In deciding to have all
/// the puzzles run from a single application, it makes sense to harmonize these types to return
/// to the caller.
#[derive(Clone, Debug)]
pub enum AocResult {
    Item(u32),
    U32List(Vec<u32>),
    UsizeList(Vec<usize>),
}

impl From<u32> for AocResult {
    fn from(item: u32) -> Self {
        Self::Item(item)
    }
}

impl From<Vec<u32>> for AocResult {
    fn from(list: Vec<u32>) -> Self {
        Self::U32List(list)
    }
}

impl From<Vec<usize>> for AocResult {
    fn from(list: Vec<usize>) -> Self {
        Self::UsizeList(list)
    }
}
