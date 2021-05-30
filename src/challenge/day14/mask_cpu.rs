use anyhow::{Context, Result};

const MASK_WIDTH: usize = 36;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Mask {
    ones: u64,
    zeros: u64,
}

impl Mask {
    pub fn apply(&self, x: u64) -> u64 {
        let x1 = x | self.ones;
        let x01 = !x1 | self.zeros;
        !x01
    }
}

pub fn parse_mask(input: &str) -> Result<Mask> {
    let mut ones = 0u64;
    let mut zeros = 0u64;

    for (idx, c) in input.chars().enumerate() {
        let set_bit = 1u64 << (MASK_WIDTH - idx - 1);
        match c {
            '0' => zeros |= set_bit,
            '1' => ones |= set_bit,
            'X' => continue,
            _ => anyhow::bail!("mask can not include char: {:?}", c),
        }
    }
    Ok(Mask { ones, zeros })
}

#[derive(Debug, Default, Clone)]
pub struct Memory {
    tape: Vec<u64>,
}

impl Memory {
    fn set(&mut self, idx: usize, value: u64) {
        let req_size = idx + 1;
        if req_size > self.tape.len() {
            self.tape.resize(req_size, 0);
        }
        self.tape[idx] = value;
    }
    pub fn sum(&self) -> u64 {
        self.tape.iter().sum()
    }
}

enum Instruction {
    SetMask(Mask),
    SetValue(usize, u64),
}

fn parse_instruction(input: &str) -> Result<Instruction> {
    const MASK_LEADER: &str = "mask = ";
    const MEM_LEADER: &str = "mem[";

    let instr = if input.starts_with(MASK_LEADER) {
        let mask_str = &input[MASK_LEADER.len()..];
        let mask = parse_mask(mask_str)?;
        Instruction::SetMask(mask)
    } else if input.starts_with(MEM_LEADER) {
        let end_addr = input
            .find("]")
            .ok_or_else(|| anyhow::anyhow!("mem instruction was malformed: {:?}", input))?;
        let addr_text = &input[MEM_LEADER.len()..end_addr];
        let addr = addr_text
            .parse::<usize>()
            .with_context(|| format!("could not parse address: {:?}", input))?;
        let value_text = &input[(end_addr + 4)..];
        let value = value_text
            .parse::<u64>()
            .with_context(|| format!("could not parse value: {:?}", input))?;

        Instruction::SetValue(addr, value)
    } else {
        anyhow::bail!("unknown instruction: {:?}", input);
    };

    Ok(instr)
}

pub struct Program(Vec<Instruction>);

impl Program {
    pub fn parse(input: &str) -> Result<Program> {
        let instrs = input
            .lines()
            .map(|l| parse_instruction(l))
            .collect::<Result<Vec<_>>>()?;
        Ok(Program(instrs))
    }
}

#[derive(Default)]
pub struct MaskCpuV1 {
    mask: Mask,
    mem: Memory,
}

impl MaskCpuV1 {
    pub fn program(&mut self, program: &Program) -> u64 {
        for instr in &program.0 {
            self.run(instr)
        }
        self.mem.sum()
    }

    fn run(&mut self, instr: &Instruction) {
        match instr {
            Instruction::SetMask(m) => self.mask = *m,
            Instruction::SetValue(idx, v) => self.mem.set(*idx, self.mask.apply(*v)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_mask() {
        let mask = parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX").unwrap();
        assert_eq!(mask.ones, 0);
        assert_eq!(mask.zeros, 0);
    }

    #[test]
    fn parse_zero_small() {
        let mask = parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX0").unwrap();
        assert_eq!(mask.ones, 0);
        assert_eq!(mask.zeros, 1);
    }
    #[test]
    fn parse_one_small() {
        let mask = parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX1").unwrap();
        assert_eq!(mask.ones, 1);
        assert_eq!(mask.zeros, 0);
    }

    #[test]
    fn parse_zero_large() {
        let mask = parse_mask("0XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX").unwrap();
        assert_eq!(mask.ones, 0);
        assert_eq!(mask.zeros, 34359738368);
    }
    #[test]
    fn parse_mixed() {
        let mask = parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX0X101X").unwrap();
        assert_eq!(mask.ones, 10);
        assert_eq!(mask.zeros, 36);
    }

    #[test]
    fn example_text() {
        let mask = parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        assert_eq!(mask.apply(11), 73);
        assert_eq!(mask.apply(101), 101);
        assert_eq!(mask.apply(0), 64);
    }
}
