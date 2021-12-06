use anyhow::Result;
use std::fmt;

mod mask_cpu;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let p = mask_cpu::Program::parse(input)?;
    let mut cpu = mask_cpu::MaskCpuV1::default();
    Ok(format!("{}", cpu.program(&p)))
}
pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let p = mask_cpu::Program::parse(input)?;
    Ok(mask_cpu::execute_v4(&p)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day14");
    const EX: &str = include_str!("../../../input/day14_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "10717676595607")
    }

    #[test]
    fn check_example_p1() {
        assert_eq!(format!("{}", part1(EX).unwrap()), "165")
    }
    #[test]
    fn verify_p2() {
        // assert_eq!(format!("{}", part2(INPUT).unwrap()), "")
    }
}
