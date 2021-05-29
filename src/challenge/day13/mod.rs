use anyhow::{Context, Result};
use std::fmt;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let (depart, busses) = parse(input)?;
    log::debug!("depart earliest: {:?}", depart);
    log::debug!("bus schedules: {:?}", busses);

    let working = busses
        .iter()
        .filter_map(|b| match b {
            BusData::BusID(x) => Some(x),
            _ => None,
        })
        .collect::<Vec<_>>();

    let (id, wait) = next_bus(depart, &working)?;
    let answer = id * wait;

    Ok(format!("{}", answer))
}
pub fn part2(input: &str) -> Result<impl fmt::Display> {
    Ok("")
}

#[derive(Debug)]
enum BusData {
    BusID(i64),
    X,
}

fn next_bus(at: i64, candidates: &[&i64]) -> Result<(i64, i64)> {
    candidates
        .iter()
        .map(|c| {
            let rem = at % *c;
            let wait = *c - rem;
            log::trace!("id: {:?}: {} % {} = {} (+{})", c, at, c, at % *c, wait);
            (**c, wait)
        })
        .min_by_key(|r| r.1)
        .ok_or_else(|| anyhow::anyhow!("no valid next bus"))
}

fn parse(input: &str) -> Result<(i64, Vec<BusData>)> {
    let mut lines = input.lines();
    let line1 = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not get first line"))?;
    let depart_time = line1
        .parse::<i64>()
        .with_context(|| format!("could not parse number: {:?}", line1))?;

    let line2 = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not get second line"))?;

    let bus_data = line2
        .split(",")
        .map(|s| {
            if s == "x" {
                Ok(BusData::X)
            } else {
                s.parse::<i64>()
                    .map(|n| BusData::BusID(n))
                    .with_context(|| format!("could not parse number: {:?}", s))
            }
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((depart_time, bus_data))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day13");
    const EX: &str = include_str!("../../../input/day13_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "207")
    }

    #[test]
    fn example_p1() {
        assert_eq!(format!("{}", part1(EX).unwrap()), "295")
    }

    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "")
    }
}
