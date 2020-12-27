use anyhow::{Context, Result};
use std::fmt;

const PREAMBLE_LEN: usize = 25;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    parse_and_report_invalid_number(input, PREAMBLE_LEN)
}
pub fn part2(input: &str) -> Result<impl fmt::Display> {
    find_weakness(input, PREAMBLE_LEN)
}

fn parse_and_report_invalid_number(input: &str, preamble: usize) -> Result<i64> {
    let buf = parse(input)?;
    find_incorrect_element(buf.as_slice(), preamble)
}

fn find_weakness(input: &str, preamble: usize) -> Result<i64> {
    let buf = parse(input)?;
    let invalid = find_incorrect_element(buf.as_slice(), preamble)?;

    let mut start = 0;
    let mut end = 2;
    let mut total = buf[0..end].iter().sum::<i64>();

    loop {
        log::trace!(
            "[{}..{}] {:?}, total={}",
            start,
            end,
            &buf[start..end],
            total
        );
        match total.cmp(&invalid) {
            std::cmp::Ordering::Less => {
                total += buf[end];
                end += 1;
            }
            std::cmp::Ordering::Greater => {
                total -= buf[start];
                start += 1;
            }
            std::cmp::Ordering::Equal => {
                let mut min = buf[start];
                let mut max = buf[start];
                for e in &buf[start + 1..end] {
                    if *e < min {
                        min = *e
                    }
                    if *e > max {
                        max = *e
                    }
                }
                return Ok(min + max);
            }
        }
    }
}

fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.parse::<i64>()
                .with_context(|| format!("not a number: {:?}", l))
        })
        .collect()
}

fn find_incorrect_element(data: &[i64], preamble: usize) -> Result<i64> {
    let mut buf = RingBuffer::new(data[0..preamble].to_owned());
    for e in &data[preamble..] {
        if !is_sum_of_two(buf.as_slice(), *e) {
            return Ok(*e);
        }
        buf.insert(*e)
    }
    anyhow::bail!("could not find an element that was invalid")
}

fn is_sum_of_two(buf: &[i64], e: i64) -> bool {
    for (idx, i) in buf.iter().enumerate() {
        let target = e - *i;
        for j in &buf[idx..] {
            if *j == target {
                return true;
            }
        }
    }
    false
}

struct RingBuffer {
    inner: Vec<i64>,
    cursor: usize,
}

impl RingBuffer {
    fn new(inner: Vec<i64>) -> RingBuffer {
        RingBuffer { inner, cursor: 0 }
    }
    fn as_slice(&self) -> &[i64] {
        self.inner.as_slice()
    }
    fn insert(&mut self, value: i64) {
        self.inner[self.cursor] = value;
        self.cursor = (self.cursor + 1) % self.inner.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day9");
    const EX: &str = include_str!("../../../input/day9_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "18272118")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "2186361")
    }
    #[test]
    fn p1ex1() {
        assert_eq!(parse_and_report_invalid_number(EX, 5).unwrap(), 127)
    }
    #[test]
    fn p2ex1() {
        assert_eq!(find_weakness(EX, 5).unwrap(), 62)
    }
}
