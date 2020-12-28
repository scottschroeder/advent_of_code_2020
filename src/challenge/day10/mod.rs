use anyhow::{Context, Result};
use std::{collections::HashMap, fmt};

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let mut adapters = parse(input)?;
    adapters.sort();
    let mut jolt_deltas = HashMap::new();
    jolt_deltas.insert(adapters[0], 1);
    *jolt_deltas.entry(3).or_insert(0) += 1;
    for diff in adapters
        .iter()
        .zip(adapters.iter().skip(1))
        .map(|(a, b)| *b - *a)
    {
        *jolt_deltas.entry(diff).or_insert(0) += 1;
    }
    log::debug!("{:#?}", jolt_deltas);
    let ones = jolt_deltas.get(&1).cloned().unwrap_or(0);
    let threes = jolt_deltas.get(&3).cloned().unwrap_or(0);
    Ok(ones * threes)
}

pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let mut adapters = parse(input)?;
    adapters.sort();
    let mut cache = HashMap::new();
    Ok(combos(0, adapters.as_slice(), &mut cache))
}

fn combos(start: i64, adapters: &[i64], cache: &mut HashMap<i64, usize>) -> usize {
    if let Some(s) = cache.get(&start) {
        return *s;
    }
    if adapters.is_empty() {
        log::trace!("start: {}, total: 1", start);
        return 1;
    }

    let mut total = 0;
    for (ix, adapter) in adapters.iter().take_while(|a| *a - start < 4).enumerate() {
        total += combos(*adapter, &adapters[ix + 1..], cache);
    }

    log::trace!(
        "start: {}, total: {} adapters: {:?}",
        start,
        total,
        &adapters[..std::cmp::min(5, adapters.len())]
    );
    cache.insert(start, total);
    total
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
    const INPUT: &str = include_str!("../../../input/day10");
    const EX1: &str = include_str!("../../../input/day10_ex1");
    const EX2: &str = include_str!("../../../input/day10_ex2");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "2080")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "6908379398144")
    }
    #[test]
    fn part1_ex1() {
        assert_eq!(format!("{}", part1(EX1).unwrap()), "35")
    }
    #[test]
    fn part1_ex2() {
        assert_eq!(format!("{}", part1(EX2).unwrap()), "220")
    }
    #[test]
    fn part2_ex1() {
        assert_eq!(format!("{}", part2(EX1).unwrap()), "8")
    }
    #[test]
    fn part2_ex2() {
        assert_eq!(format!("{}", part2(EX2).unwrap()), "19208")
    }
}
