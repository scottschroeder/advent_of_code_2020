use crate::gameconsole::{Code, Instruction, Machine, Operation};
use anyhow::Result;
use std::{collections::HashSet, fmt};

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let mut vm = Machine::new(Code::parse(input)?);
    log::trace!("{:#?}", vm);
    let mut seen = HashSet::new();
    while seen.insert(vm.ix) {
        vm.step()
    }
    Ok(vm.acc)
}
pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let code = Code::parse(input)?;
    for (ix, instr) in code.data.iter().enumerate() {
        let new_op = match instr.op {
            Operation::Nop => Operation::Jmp,
            Operation::Acc => continue,
            Operation::Jmp => Operation::Nop,
        };
        let mut new_code = code.clone();
        new_code.data[ix] = Instruction {
            op: new_op,
            arg: instr.arg,
        };
        let vm = Machine::new(new_code);
        if let Ok(acc) = check_terminate(vm) {
            return Ok(acc);
        }
    }
    anyhow::bail!("could not find suitible instruction to replace");
}

fn check_terminate(mut vm: Machine) -> Result<i64> {
    let terminate = vm.code.len();
    log::trace!("{:#?}", vm);
    let mut seen = HashSet::new();
    while seen.insert(vm.ix) {
        vm.step();
        if vm.ix == terminate {
            return Ok(vm.acc);
        }
    }
    anyhow::bail!("cycle")
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day8");
    const EX: &str = include_str!("../../../input/day8_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "1553")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "1877")
    }

    #[test]
    fn pt1_ex() {
        assert_eq!(format!("{}", part1(EX).unwrap()), "5")
    }
    #[test]
    fn pt2_ex() {
        assert_eq!(format!("{}", part2(EX).unwrap()), "8")
    }
}
