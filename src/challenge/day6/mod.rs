use anyhow::Result;
use aoc::bitset::alphabet::{Key, KeySet};
use std::fmt;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let colletor = GroupCollector {
        inner: parse(input),
        join: |k1: KeySet, k2: KeySet| k1.union(k2),
    };
    Ok(colletor.map(|g| g.len()).sum::<usize>())
}
pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let colletor = GroupCollector {
        inner: parse(input),
        join: |k1: KeySet, k2: KeySet| k1.intersect(k2),
    };
    Ok(colletor.map(|g| g.len()).sum::<usize>())
}

struct GroupCollector<I, F> {
    inner: I,
    join: F,
}

impl<I: Iterator<Item = Option<KeySet>>, F: Fn(KeySet, KeySet) -> KeySet> Iterator
    for GroupCollector<I, F>
{
    type Item = KeySet;

    fn next(&mut self) -> Option<Self::Item> {
        let mut collector = None;
        while let Some(Some(answer)) = self.inner.next() {
            if let Some(group) = collector {
                let p = group;
                collector = Some((self.join)(group, answer));
                log::trace!("join: {:?} + {:?} = {:?}", p, answer, group);
            } else {
                collector = Some(answer)
            }
        }
        collector
    }
}

fn parse(input: &str) -> impl Iterator<Item = Option<KeySet>> + '_ {
    input.lines().map(|s| {
        if s.is_empty() {
            None
        } else {
            Some(parse_line(s))
        }
    })
}
fn parse_line(input: &str) -> KeySet {
    let mut set = KeySet::default();
    for c in input.chars() {
        set = set.insert(Key::from(c))
    }
    set
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day6");
    const EX: &str = include_str!("../../../input/day6_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "6161")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "2971")
    }
    #[test]
    fn p1_ex() {
        assert_eq!(format!("{}", part1(EX).unwrap()), "11")
    }
}
