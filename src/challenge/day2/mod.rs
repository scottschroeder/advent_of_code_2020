use anyhow::{anyhow as ah, Context, Result};
use std::{convert::TryFrom, fmt, str::FromStr};

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let pws = parse(input)?;
    Ok(pws.iter().filter(|pe| pe.check()).count())
}
pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let pws = parse(input)?;
    Ok(pws.iter().filter(|pe| pe.check_v2()).count())
}

fn parse(input: &str) -> Result<Vec<PasswordEntry<'_>>> {
    input
        .lines()
        .map(|s| {
            PasswordEntry::try_from(s).with_context(|| format!("parse password entry: '{}'", s))
        })
        .collect()
}

#[derive(Debug)]
struct PasswordPolicy {
    min: u32,
    max: u32,
    c: char,
}

impl PasswordPolicy {
    fn check(&self, s: &str) -> bool {
        let count = s.chars().filter(|c| *c == self.c).count() as u32;
        count >= self.min && count <= self.max
    }
    fn check_v2(&self, s: &str) -> bool {
        let count = s
            .chars()
            .enumerate()
            .filter(|(idx, c)| {
                let pos = (*idx + 1) as u32;
                (pos == self.min || pos == self.max) && *c == self.c
            })
            .count() as u32;
        count == 1
    }
}

impl FromStr for PasswordPolicy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<PasswordPolicy, Self::Err> {
        let mut split = s.splitn(2, ' ').into_iter();
        let minmax_str = split.next().ok_or_else(|| ah!("no text in policy"))?;
        let char = split
            .next()
            .ok_or_else(|| ah!("no space in '<min>-<max> <char>'"))?;
        let c = char.chars().nth(0).ok_or_else(|| ah!("chars was empty"))?;

        let mut minmax_split = minmax_str.splitn(2, '-').into_iter();
        let min_str = minmax_split
            .next()
            .ok_or_else(|| ah!("minmax string empty"))?;
        let max_str = minmax_split
            .next()
            .ok_or_else(|| ah!("minmax string had no hyphen"))?;

        let min = min_str.parse::<u32>()?;
        let max = max_str.parse::<u32>()?;

        Ok(PasswordPolicy { min, max, c })
    }
}

#[derive(Debug)]
struct PasswordEntry<'a> {
    policy: PasswordPolicy,
    passwd: &'a str,
}

impl<'a> PasswordEntry<'a> {
    fn check(&self) -> bool {
        self.policy.check(self.passwd)
    }
    fn check_v2(&self) -> bool {
        self.policy.check_v2(self.passwd)
    }
}

impl<'a> TryFrom<&'a str> for PasswordEntry<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<PasswordEntry<'a>, Self::Error> {
        let mut split = s.splitn(2, ':').into_iter();
        let policy_str = split.next().ok_or_else(|| ah!("no text in entry"))?;
        let password = split.next().ok_or_else(|| ah!("no ':' in entry"))?.trim();
        let policy = PasswordPolicy::from_str(policy_str).context("could not parse policy")?;
        let entry = PasswordEntry {
            policy,
            passwd: password,
        };
        log::trace!("parse: {:?} -> {:?}", s, entry);
        Ok(entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day2");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "422")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "451")
    }
}
