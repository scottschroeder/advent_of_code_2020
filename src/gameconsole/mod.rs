use anyhow::{anyhow as ah, Context, Result};
use std::{fmt, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Nop,
    Acc,
    Jmp,
}
impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Nop => write!(f, "nop"),
            Operation::Acc => write!(f, "acc"),
            Operation::Jmp => write!(f, "jmp"),
        }
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        match s {
            "nop" => Ok(Operation::Nop),
            "acc" => Ok(Operation::Acc),
            "jmp" => Ok(Operation::Jmp),
            _ => Err(ah!("unrecognized operation: {:?}", s)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub op: Operation,
    pub arg: i64,
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (sign, abs) = if self.arg < 0 {
            ('-', -1 * self.arg)
        } else {
            ('+', self.arg)
        };
        write!(f, "{:?} {}{}", self.op, sign, abs)
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut split = s.splitn(2, " ").into_iter();
        let op_str = split.next().ok_or_else(|| ah!("instruction was empty"))?;
        let arg_str = split
            .next()
            .ok_or_else(|| ah!("instruction was not space separated"))?;
        let op = Operation::from_str(op_str)?;
        let arg = arg_str
            .parse::<i64>()
            .with_context(|| format!("{:?} was not an integer", arg_str))?;
        Ok(Instruction { op, arg })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Code {
    pub data: Vec<Instruction>,
}

impl Code {
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn parse(s: &str) -> Result<Code> {
        Ok(Code {
            data: s
                .lines()
                .map(|l| Instruction::from_str(l))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Machine {
    pub acc: i64,
    pub ix: usize,
    pub code: Code,
}

impl Machine {
    pub fn new(code: Code) -> Machine {
        Machine {
            acc: 0,
            ix: 0,
            code,
        }
    }

    pub fn step(&mut self) {
        let instr = self.code.data[self.ix];

        log::trace!("acc: {} ix: {} => {:?}", self.acc, self.ix, instr);

        match instr.op {
            Operation::Nop => {
                self.ix += 1;
            }
            Operation::Acc => {
                self.acc += instr.arg;
                self.ix += 1;
            }
            Operation::Jmp => {
                if instr.arg < 0 {
                    let arg = (-1 * instr.arg) as usize;
                    assert!(
                        arg < self.ix,
                        "attempt to jump out of bounds {} - {}",
                        self.ix,
                        arg
                    );
                    self.ix -= arg
                } else {
                    self.ix += instr.arg as usize;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instrs() {
        assert_eq!(
            Instruction::from_str("nop +0").unwrap(),
            Instruction {
                op: Operation::Nop,
                arg: 0
            }
        );
        assert_eq!(
            Instruction::from_str("nop -0").unwrap(),
            Instruction {
                op: Operation::Nop,
                arg: 0
            }
        );
        assert_eq!(
            Instruction::from_str("acc +1").unwrap(),
            Instruction {
                op: Operation::Acc,
                arg: 1
            }
        );
        assert_eq!(
            Instruction::from_str("acc +19").unwrap(),
            Instruction {
                op: Operation::Acc,
                arg: 19
            }
        );
        assert_eq!(
            Instruction::from_str("acc -4").unwrap(),
            Instruction {
                op: Operation::Acc,
                arg: -4
            }
        );
        assert_eq!(
            Instruction::from_str("jmp +1").unwrap(),
            Instruction {
                op: Operation::Jmp,
                arg: 1
            }
        );
    }
}
