use anyhow::{Context, Result};
use std::fmt;

mod rem_offset;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let (depart, busses) = parse(input)?;
    log::debug!("depart earliest: {:?}", depart);
    log::debug!("bus schedules: {:?}", busses);

    let working = busses.iter().filter_map(|b| b.as_ref()).collect::<Vec<_>>();

    let (id, wait) = next_bus(depart, &working)?;
    let answer = id * wait;

    Ok(format!("{}", answer))
}
pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let (_, busses) = parse(input)?;
    let x = rem_offset::chain_offset(busses.as_slice())?;
    Ok(format!("{}", x))
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

fn parse(input: &str) -> Result<(i64, Vec<Option<i64>>)> {
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

    let bus_data = parse_schedule(line2)?;

    Ok((depart_time, bus_data))
}

fn parse_schedule(input: &str) -> Result<Vec<Option<i64>>> {
    input
        .split(",")
        .map(|s| {
            if s == "x" {
                Ok(None)
            } else {
                s.parse::<i64>()
                    .map(|n| Some(n))
                    .with_context(|| format!("could not parse number: {:?}", s))
            }
        })
        .collect::<Result<Vec<_>>>()
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
    fn example_p2() {
        assert_eq!(format!("{}", part2(EX).unwrap()), "1068781")
    }

    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "530015546283687")
    }

    fn earliest_seq_test(input: &str, expected: i64) {
        let data = parse_schedule(input).unwrap();
        let early = rem_offset::chain_offset(data.as_slice()).unwrap();
        assert_eq!(early, expected);
    }

    #[test]
    fn earliest_seq_ex1() {
        earliest_seq_test("17,x,13,19", 3417);
    }
    #[test]
    fn earliest_seq_ex2() {
        earliest_seq_test("67,7,59,61", 754018);
    }
    #[test]
    fn earliest_seq_ex3() {
        earliest_seq_test("67,x,7,59,61", 779210);
    }
    #[test]
    fn earliest_seq_ex4() {
        earliest_seq_test("67,7,x,59,61", 1261476);
    }
    #[test]
    fn earliest_seq_ex5() {
        earliest_seq_test("1789,37,47,1889", 1202161486);
    }
}
