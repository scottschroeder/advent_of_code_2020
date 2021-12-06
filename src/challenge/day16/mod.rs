use anyhow::{Context, Result};
use std::fmt;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let (classes, my_ticket, other_tickets) = parse(input)?;
    log::trace!("classes: {:?}", classes);
    log::trace!("my_ticket: {:?}", my_ticket);
    log::trace!("other_tickets: {:?}", other_tickets);

    let error_rate = other_tickets
        .iter()
        .flat_map(|t| t.0.iter())
        .filter(|value| !classes.iter().any(|c| c.contains(*value)))
        .sum::<i64>();
    Ok(format!("{:?}", error_rate))
}

pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let (classes, my_ticket, other_tickets) = parse(input)?;

    let valid_tickets = other_tickets
        .into_iter()
        .filter(|ticket| {
            !ticket
                .0
                .iter()
                .any(|value| !classes.iter().any(|c| c.contains(value)))
        })
        .collect::<Vec<_>>();

    Ok(format!("{:?}", 0))
}

fn extract_split<'a>(segments: &mut impl Iterator<Item = &'a str>) -> Result<&'a str> {
    segments
        .next()
        .ok_or_else(|| anyhow::anyhow!("segment missing"))
}

fn parse(input: &str) -> Result<(Vec<Class>, Ticket, Vec<Ticket>)> {
    let mut segments = input.split("\n\n");
    let mut get_chunk = || {
        segments
            .next()
            .ok_or_else(|| anyhow::anyhow!("unable to parse: {:?}", input))
    };
    let classes_str = get_chunk()?;
    let my_ticket_str = get_chunk()?;
    let other_tikets_str = get_chunk()?;

    let classes = classes_str
        .lines()
        .map(parse_class)
        .collect::<Result<Vec<_>>>()?;

    let ticket = my_ticket_str
        .lines()
        .skip(1)
        .map(parse_ticket)
        .next()
        .ok_or_else(|| anyhow::anyhow!("my ticket did not exist"))??;

    let other_tickets = other_tikets_str
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect::<Result<Vec<Ticket>>>()?;

    Ok((classes, ticket, other_tickets))
}

fn parse_class(input: &str) -> Result<Class> {
    let mut segments = input.split(':');
    let class_name = extract_split(&mut segments).context("class name")?;
    let ranges = extract_split(&mut segments).context("class name")?.trim();
    let mut segments = ranges.split(" or ");
    let r1_str = extract_split(&mut segments).context("range #1")?;
    let r2_str = extract_split(&mut segments).context("range #2")?;

    let r1 = parse_range(r1_str)?;
    let r2 = parse_range(r2_str)?;

    log::trace!("class {:?} r1: {:?} r2: {:?}", class_name, r1, r2);

    Ok(Class {
        name: class_name,
        r1,
        r2,
    })
}

fn parse_range(input: &str) -> Result<Range> {
    let mut segments = input.split('-');
    let start_str = extract_split(&mut segments).context("range_start")?;
    let end_str = extract_split(&mut segments).context("range_end")?.trim();
    Ok(Range {
        start: start_str
            .parse::<i64>()
            .with_context(|| format!("could not parse number: {:?}", start_str))?,
        end: end_str
            .parse::<i64>()
            .with_context(|| format!("could not parse number: {:?}", end_str))?,
    })
}

fn parse_ticket(input: &str) -> Result<Ticket> {
    Ok(Ticket(
        input
            .split(',')
            .map(|s| {
                s.parse::<i64>()
                    .with_context(|| format!("unable to parse number from: {:?}", input))
            })
            .collect::<Result<Vec<i64>>>()?,
    ))
}

#[derive(Debug, Clone)]
struct Class<'a> {
    name: &'a str,
    r1: Range,
    r2: Range,
}
#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn contains(self, x: &i64) -> bool {
        (self.start..=self.end).contains(x)
    }
}

impl<'a> Class<'a> {
    fn contains(&self, x: &i64) -> bool {
        self.r1.contains(x) || self.r2.contains(x)
    }
}

#[derive(Debug, Clone)]
struct Ticket(Vec<i64>);

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day16");
    const EX: &str = include_str!("../../../input/day16_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "27870")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "0")
    }

    #[test]
    fn check_p1_example() {
        assert_eq!(format!("{}", part1(EX).unwrap()), "71")
    }

    #[test]
    fn range_inclusive() {
        let r = Range { start: 3, end: 5 };
        assert!(r.contains(&3));
        assert!(r.contains(&4));
        assert!(r.contains(&5));
    }

    #[test]
    fn range_exclude() {
        let r = Range { start: 3, end: 5 };
        assert!(!r.contains(&2));
        assert!(!r.contains(&6));
    }
}
