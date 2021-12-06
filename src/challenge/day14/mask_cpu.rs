use std::{fmt, u64};

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

    fn unset(&self) -> u64 {
        !(self.ones | self.zeros)
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

fn parse_floating(input: &str) -> Result<FloatingAddr> {
    let mut start = 0u64;
    let mut float = 0u64;

    for (idx, c) in input.chars().enumerate() {
        let set_bit = 1u64 << (MASK_WIDTH - idx - 1);
        match c {
            '0' => continue,
            '1' => start |= set_bit,
            'X' => float |= set_bit,
            _ => anyhow::bail!("addr can not include char: {:?}", c),
        }
    }
    Ok(FloatingAddr {
        start,
        blackout: !float,
    })
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

#[derive(Clone, Copy, PartialEq)]
struct FloatingAddr {
    start: u64,
    blackout: u64,
}

impl FloatingAddr {
    #[inline(always)]
    fn float(self) -> u64 {
        !self.blackout & 0xF_FFFF_FFFF
    }

    fn max_value(self) -> u64 {
        log::trace!(
            "start: {:0b} blackout: {:0b}, float: {:0b}",
            self.start,
            self.blackout,
            self.float()
        );
        self.start | self.float()
    }

    fn contains(self, addr: u64) -> bool {
        self.blackout & addr == self.start
    }

    fn total_addrs(self) -> u64 {
        let fbits = self.float().count_ones() as u64;
        1 << fbits
    }

    fn union(self, other: FloatingAddr) -> FloatingAddr {
        let new_float = self.float() | other.float() | (self.start ^ other.start);

        FloatingAddr {
            start: self.start & other.start,
            blackout: !new_float,
        }
    }
    fn intersection(self, other: FloatingAddr) -> Option<FloatingAddr> {
        let fixed_lhs = self.start & other.blackout;
        let fixed_rhs = other.start & self.blackout;
        if (fixed_lhs ^ fixed_rhs) > 0 {
            return None;
        }

        Some(FloatingAddr {
            start: self.start | other.start,
            blackout: self.blackout | other.blackout,
        })
    }
}

impl fmt::Debug for FloatingAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let float = self.float();
        for b in 0..MASK_WIDTH {
            if (float >> (MASK_WIDTH - b - 1)) & 1 == 1 {
                write!(f, "X")?;
            } else {
                let mb = (self.start >> (MASK_WIDTH - b - 1)) & 1;
                write!(f, "{}", mb)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct FloatSet {
    addr: FloatingAddr,
    value: u64,
}

pub fn execute_v4(p: &Program) -> Result<u64> {
    let vfs = translate_addr(p.0.as_slice());

    let mut total = 0;
    let mut seen = None;

    for x in &vfs {
        let (new_addrs, new_seen) = count_fresh_addrs(x.addr, seen);
        total += new_addrs * x.value;
        seen = Some(new_seen)
    }

    log::debug!("sum: {:?}", total);
    Ok(total)
}

fn count_fresh_addrs(addr: FloatingAddr, seen: Option<FloatingAddr>) -> (u64, FloatingAddr) {
    log::trace!(
        "Already Written: {:?} (n={})",
        seen,
        seen.map(|s| s.total_addrs()).unwrap_or(0)
    );
    let intersection = seen.and_then(|s| s.intersection(addr));
    let intersection_count = intersection.map(|s| s.total_addrs()).unwrap_or(0);
    log::trace!(
        "Intersection: {:?} (n={})",
        intersection,
        intersection_count
    );
    let new_addrs = addr.total_addrs() - intersection_count;
    log::trace!("New Addresses: {:?} (n={})", addr, new_addrs);
    let new_seen = seen.map(|s| s.union(addr)).unwrap_or(addr);
    (new_addrs, new_seen)
}

pub fn execute_v2(p: &Program) -> Result<u64> {
    let vfs = translate_addr(p.0.as_slice());

    let mut touched_addrs = 0;
    for x in &vfs {
        let addrs = x.addr.total_addrs();
        log::trace!("{:?} (n={})", x, addrs);
        touched_addrs += addrs;
    }

    let max_addr: u64 = vfs
        .iter()
        .map(|fs| {
            let mv = fs.addr.max_value();
            log::trace!("addr: {:?}, max {:?}", fs.addr, mv);
            mv
        })
        .max()
        .ok_or_else(|| anyhow::anyhow!("empty program"))?;

    log::debug!("max addr: {:?}", max_addr);
    log::debug!("touched addrs: {:?}", touched_addrs);

    let mut sum = 0;
    for addr in 0..max_addr + 1 {
        'instr: for instr in &vfs {
            if instr.addr.contains(addr) {
                sum += instr.value;
                break 'instr;
            }
        }
    }
    log::debug!("sum: {:?}", sum);
    Ok(sum)
}

fn translate_addr(program: &[Instruction]) -> Vec<FloatSet> {
    let mut mask = Mask::default();
    let mut float_program = Vec::with_capacity(program.len());
    for instr in program {
        match instr {
            Instruction::SetMask(m) => mask = *m,
            Instruction::SetValue(a, v) => {
                let a = *a as u64 | mask.ones;
                let blackout = !mask.unset();

                let addr = FloatingAddr {
                    start: a & blackout,
                    blackout,
                };

                float_program.push(FloatSet { addr, value: *v })
            }
        }
    }
    float_program.reverse();
    float_program
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

    fn check_union(a: &str, b: &str, e: &str) {
        let a1 = parse_floating(a).unwrap();
        let a2 = parse_floating(b).unwrap();
        assert_eq!(a1.union(a2), parse_floating(e).unwrap());
    }

    fn check_intersection(a: &str, b: &str, e: Option<&str>) {
        let a1 = parse_floating(a).unwrap();
        let a2 = parse_floating(b).unwrap();
        let e = e.map(|s| parse_floating(s).unwrap());
        assert_eq!(a1.intersection(a2), e);
    }

    #[test]
    fn union_empty() {
        check_union(
            "000000000000000000000000000000000000",
            "000000000000000000000000000000000000",
            "000000000000000000000000000000000000",
        );
    }
    #[test]
    fn union_same() {
        check_union(
            "000000000000000000000000000000001010",
            "000000000000000000000000000000001010",
            "000000000000000000000000000000001010",
        );
    }
    #[test]
    fn union_same_float() {
        check_union(
            "000000000000000000000000000000001X10",
            "000000000000000000000000000000001X10",
            "000000000000000000000000000000001X10",
        );
    }
    #[test]
    fn union_join() {
        check_union(
            "000000000000000000000000000000001X10",
            "000000000000000000000000000000001X00",
            "000000000000000000000000000000001XX0",
        );
    }

    #[test]
    fn union_float_override() {
        check_union(
            "000000000000000000000000000000001X10",
            "000000000000000000000000000000001XXX",
            "000000000000000000000000000000001XXX",
        );
    }

    #[test]
    fn intersection_empty() {
        check_intersection(
            "000000000000000000000000000000000000",
            "000000000000000000000000000000000000",
            Some("000000000000000000000000000000000000"),
        );
    }
    #[test]
    fn intersection_same() {
        check_intersection(
            "000000000000000000000000000000001010",
            "000000000000000000000000000000001010",
            Some("000000000000000000000000000000001010"),
        );
    }
    #[test]
    fn intersection_same_float() {
        check_intersection(
            "000000000000000000000000000000001X10",
            "000000000000000000000000000000001X10",
            Some("000000000000000000000000000000001X10"),
        );
    }
    #[test]
    fn intersection_join() {
        check_intersection(
            "000000000000000000000000000000001X10",
            "000000000000000000000000000000001X00",
            None,
        );
    }

    #[test]
    fn intersection_float_override() {
        check_intersection(
            "000000000000000000000000000000001X10",
            "000000000000000000000000000000001XXX",
            Some("000000000000000000000000000000001X10"),
        );
    }

    #[test]
    fn count_touched_addrs_zeros() {
        let a = parse_floating("000000000000000000000000000000000000").unwrap();
        assert_eq!(a.total_addrs(), 1)
    }

    #[test]
    fn count_touched_addrs_one() {
        let a = parse_floating("000000000000000000000000000000000001").unwrap();
        assert_eq!(a.total_addrs(), 1)
    }

    #[test]
    fn count_touched_addrs_random() {
        let a = parse_floating("000000000100100001001110011001000001").unwrap();
        assert_eq!(a.total_addrs(), 1)
    }
    #[test]
    fn count_touched_addrs_single_float() {
        let a = parse_floating("0000000001001000010X1110011001000001").unwrap();
        assert_eq!(a.total_addrs(), 2)
    }
    #[test]
    fn count_touched_addrs_double_float() {
        let a = parse_floating("0000X00001001000010X1110011001000001").unwrap();
        assert_eq!(a.total_addrs(), 4)
    }
}
