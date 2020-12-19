use anyhow::Result;
use std::fmt;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    Ok("")
}
pub fn part2(input: &str) -> Result<impl fmt::Display> {
    Ok("")
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day1");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "")
    }
}
