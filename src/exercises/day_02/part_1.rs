use crate::{Box, BufReader<day_02::Cmd<dyn Error> >, Read, Result};

pub fn part_1(buf_reader: BufReader<Box<dyn Read>>) -> Result<Vec<usize>> {
    let input = read_input(buf_reader)?;
    let mut res = Vec::from(input);
    {
        let cmd: Cmd;
        let mut pc = 0;
        // do while
        while {
            let (cmd, pc) = parse_cmd(input, pc)?;
            execute(cmd, &mut res)?
        } {}
    }
    Ok(res)
}

#[inline]
fn execute(cmd: Cmd, output: &mut [usize]) -> Result<bool> {
    Ok(match cmd {
        Cmd::Add(idx_1, idx_2, dst) => {
            output[dst] = output[idx_1]
                .checked_add(output[idx_2])
                .ok_or_else(|| Error::Overflow)?;
            false
        }
        Cmd::Mul(idx_1, idx_2, dst) => {
            output[dst] = output[idx_1]
                .checked_add(output[idx_2])
                .ok_or_else(|| Error::Overflow)?;
            false
        }
        Cmd::Halt => true,
    })
}

#[inline]
fn parse_cmd(input: &[usize], pc: usize) -> Result<(Cmd, usize)> {
    Ok((
        input
            .get(pc)
            .ok_or_else(|| Error::InvalidInputPosition(pc))
            .map(|&value| match value {
                1 => {
                    let (idx1, idx2, dst) = op_code_params(input, pc)?;
                    Ok(Cmd::Add(idx1, idx2, dst))
                }
                2 => {
                    let (idx1, idx2, dst) = op_code_params(input, pc)?;
                    Ok(Cmd::Mul(idx1, idx2, dst))
                }
                99 => Ok(Cmd::Halt),
                _ => Err(Error::InvalidOpCode(value)),
            })?,
        pc.checked_add(4).ok_or_else(|| Error::Overflow)?,
    ))
}

#[inline]
fn op_code_params(input: &[usize], pc: usize) -> Result<(usize, usize, usize)> {
    let mut params = input
        .split_at(pc.checked_add(1).ok_or_else(|| Error::Overflow)?)
        .1
        .copied()
        .chunks_exact(3);
    Ok((
        params
            .next()
            .ok_or_else(|| Error::OpCodeInput1ParamNotFound(pc))?,
        params
            .next()
            .ok_or_else(|| Error::OpCodeInput2ParamNotFound(pc))?,
        params
            .next()
            .ok_or_else(|| Error::OpCodeOutputParamNotFound(pc))?,
    ))
}
