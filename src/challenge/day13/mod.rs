use anyhow::{Context, Result};
use std::fmt;

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
    let x = chain_offset(busses.as_slice())?;
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

pub fn chain_offset(c: &[Option<i64>]) -> Result<i64> {
    let init = match c.iter().next() {
        Some(Some(x)) => *x,
        _ => anyhow::bail!("chain must start with valid entry"),
    };

    struct State {
        start: i64,
        interval: i64,
    }

    let mut state = State {
        start: 0,
        interval: init,
    };

    for (idx, id) in c.iter().enumerate().skip(1).filter_map(|(idx, id)| {
        if let Some(id) = id {
            Some((idx, *id))
        } else {
            None
        }
    }) {
        let (offset, period) = offset_rem(state.interval, id, idx as i64, state.start)?;
        state.start = offset;
        state.interval = period;
    }

    Ok(state.start)
}

pub fn offset_rem(
    current_period: i64,
    add_period: i64,
    target_offset: i64,
    start_time: i64,
) -> Result<(i64, i64)> {
    let period = current_period * add_period;

    let overlap_period = aoc::math::inverse_mod(current_period, add_period).ok_or_else(|| {
        anyhow::anyhow!(
            "two numbers ({} & {}) do not have an inverse mod, so won't ever align",
            current_period,
            add_period
        )
    })?;

    let overlap_offset = start_time % add_period;
    let overlap_cycle_target = add_period - target_offset;
    let wait_cycles = overlap_cycle_target - overlap_offset;
    let mut new_start_time = (wait_cycles * overlap_period * current_period + start_time) % period;

    while new_start_time < 0 {
        new_start_time += period
    }

    log::debug!(
        "start: {}, period: {}",
        new_start_time,
        current_period * add_period
    );
    Ok((new_start_time, period))
}

#[allow(dead_code)] // debug printing
pub fn print_grid(ids: &[&i64], start: usize, end: usize) {
    print!("time\t");
    for id in ids {
        print!("{}\t", id);
    }
    println!("");
    for idx in start..end {
        print!("{}\t", idx);

        for id in ids {
            if (idx as i64) % *id == 0 {
                print!("D\t")
            } else {
                print!(".\t")
            }
        }
        println!("");
    }
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
        let early = chain_offset(data.as_slice()).unwrap();
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

    #[test]
    fn non_offset_consecutive_small_large() {
        assert_eq!(offset_rem(3, 5, 1, 0).unwrap(), (9, 15));
    }

    #[test]
    fn offset_consecutive() {
        assert_eq!(offset_rem(15, 7, 2, 9).unwrap(), (54, 105));
    }

    #[test]
    fn non_offset_skip_large_small() {
        assert_eq!(offset_rem(17, 13, 2, 0).unwrap(), (102, 221));
    }

    #[test]
    fn offset_large_number() {
        assert_eq!(
            offset_rem(166439, 19, 7, 70147).unwrap(),
            (1068781, 3162341)
        );
    }

    #[test]
    fn offset() {
        assert_eq!(offset_rem(221, 19, 3, 102).unwrap(), (3417, 4199));
    }

    #[test]
    fn chain() {
        let data = vec![Some(17), None, Some(13), Some(19)];
        assert_eq!(chain_offset(data.as_slice()).unwrap(), 3417)
    }
}
