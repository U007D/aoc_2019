#[repr(usize)]
pub enum Cmd {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Halt,
}
