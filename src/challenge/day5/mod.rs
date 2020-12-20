use anyhow::Result;
use std::{cmp, fmt};

const ROW_SIZE: usize = 7;
const COL_SIZE: usize = 3;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let seats = parse(input)?;
    log::trace!("{:?}", seats);

    seats
        .iter()
        .map(|s| s.seat_id())
        .max()
        .ok_or_else(|| anyhow::anyhow!("there were no seats"))
}
pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let seats = parse(input)?;
    let mut max_id = None;
    let mut min_id = None;
    let mut sum = 0;
    for s in &seats {
        let id = s.seat_id();
        max_id = Some(cmp::max(id, max_id.unwrap_or(id)));
        min_id = Some(cmp::min(id, min_id.unwrap_or(id)));
        sum += id
    }
    let (max_id, min_id) = max_id
        .zip(min_id)
        .ok_or_else(|| anyhow::anyhow!("there were no seats"))?;

    let expected_sum = gauss_sum(min_id - 1, max_id);
    Ok(expected_sum - sum)
}

fn gauss_sum(n1: u32, n2: u32) -> u32 {
    (n2 * (n2 + 1) - (n1 * (n1 + 1))) / 2
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Seat {
    row: u8,
    column: u8,
}

impl Seat {
    fn seat_id(self) -> u32 {
        self.row as u32 * 8 + self.column as u32
    }
}

fn parse(input: &str) -> Result<Vec<Seat>> {
    input.lines().map(|pass| parse_seat(pass)).collect()
}

fn parse_seat(text: &str) -> Result<Seat> {
    let mut row = 0u8;
    let mut column = 0u8;
    if text.len() != 10 {
        anyhow::bail!("invalid boarding pass length")
    }
    for row_mask in text[0..7]
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| match c {
            'F' => None,
            'B' => Some(Ok(1u8 << (ROW_SIZE - 1 - idx))),
            _ => Some(Err(anyhow::anyhow!("unrecognized encoding: {:?}", c))),
        })
    {
        row |= row_mask?;
    }

    for col_mask in text[7..]
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| match c {
            'L' => None,
            'R' => Some(Ok(1u8 << (COL_SIZE - 1 - idx))),
            _ => Some(Err(anyhow::anyhow!("unrecognized encoding: {:?}", c))),
        })
    {
        column |= col_mask?;
    }

    Ok(Seat { row, column })
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day5");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "926")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "657")
    }

    fn test_single_pass(pass: &str, row: u8, col: u8, id: u32) {
        let s = parse_seat(pass).unwrap();
        assert_eq!(
            s,
            Seat {
                row: row,
                column: col
            }
        );
        assert_eq!(s.seat_id(), id,)
    }

    #[test]
    fn parse_example_passes() {
        test_single_pass("FBFBBFFRLR", 44, 5, 357);
        test_single_pass("BFFFBBFRRR", 70, 7, 567);
        test_single_pass("FFFBBBFRRR", 14, 7, 119);
        test_single_pass("BBFFBBFRLL", 102, 4, 820);
    }
}
