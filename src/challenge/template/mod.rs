use anyhow::{Context, Result};
use std::fmt;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let x = 0;
    Ok(format!("{:?}", x))
}

pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let x = 0;
    Ok(format!("{:?}", x))
}

fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.parse::<i64>()
                .with_context(|| format!("could not parse number: {:?}", l))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day1");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "0")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "0")
    }
}
