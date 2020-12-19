use anyhow::{anyhow as ah, Context, Result};
use std::fmt;

const EXPENSE_TARGET: i64 = 2020;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let expenses = parse(input)?;
    pt1_impl(expenses)
}
pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let expenses = parse(input)?;
    pt2_impl(expenses)
}

fn pt1_impl(mut input: Vec<i64>) -> Result<i64> {
    input.sort();
    let (a, b) = find_expenses(input.as_slice(), EXPENSE_TARGET)
        .ok_or_else(|| ah!("could not find valid expenses"))?;
    let r = a * b;
    Ok(r)
}

fn pt2_impl(mut input: Vec<i64>) -> Result<i64> {
    input.sort();
    find_triple_expenses(input.as_slice()).ok_or_else(|| ah!("could not find valid expenses"))
}

fn find_triple_expenses(input: &[i64]) -> Option<i64> {
    for (idx, e) in input.iter().enumerate() {
        let target = EXPENSE_TARGET - *e;
        if let Some((a, b)) = find_expenses(&input[idx..], target) {
            return Some(a * b * e);
        }
    }
    None
}

fn find_expenses(input: &[i64], target: i64) -> Option<(i64, i64)> {
    let mut iter_low = 0usize;
    let mut iter_high = input.len() - 1;
    while iter_low < iter_high {
        let target_high = target - input[iter_low];
        while input[iter_high] > target_high {
            iter_high -= 1;
        }
        if input[iter_high] == target_high {
            return Some((input[iter_low], input[iter_high]));
        }
        let target_low = target - target_high;
        while input[iter_low] < target_low {
            iter_low += 1;
        }
        if input[iter_high] == target_high {
            return Some((input[iter_low], input[iter_high]));
        }
        iter_low += 1;
    }
    None
}

fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .split_ascii_whitespace()
        .map(|x| {
            x.parse::<i64>()
                .with_context(|| format!("could not parse: {}", x))
        })
        .collect::<Result<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day1");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "712075")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "145245270")
    }
    #[test]
    fn ex1_res() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(pt1_impl(input).unwrap(), 514579)
    }
    #[test]
    fn ex2_res() {
        let input = vec![1722, 979, 366, 299, 675, 1456];
        assert_eq!(pt2_impl(input).unwrap(), 241861950)
    }
}
