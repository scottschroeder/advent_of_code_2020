use anyhow::{Context, Result};
use std::{collections::HashMap, fmt};

const PT1_SPOKEN: u32 = 2020;
const PT2_SPOKEN: u32 = 30000000;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let starting = parse_input(input)?;
    Ok(number_at(&starting, PT1_SPOKEN))
}

pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let starting = parse_input(input)?;
    Ok(number_at(&starting, PT2_SPOKEN))
}

fn number_at(starting: &[u32], target: u32) -> u32 {
    let mut game = MemoryGame::init(&starting);
    loop {
        let (turn, spoken) = game.turn();
        if turn == target {
            return spoken;
        }
    }
}

fn parse_input(s: &str) -> Result<Vec<u32>> {
    s.split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.split(','))
        .flatten()
        .map(|s| {
            s.parse::<u32>()
                .map_err(|e| anyhow::format_err!("could not parse {:?}: {}", s, e))
        })
        .collect::<Result<Vec<_>>>()
}

#[derive(Debug)]
struct MemoryGame {
    history: HashMap<u32, u32>,
    counter: u32,
    last: u32,
}

impl MemoryGame {
    fn init(starting: &[u32]) -> MemoryGame {
        let mut history = HashMap::new();
        let counter = starting.len();
        for (idx, x) in starting.iter().enumerate() {
            let c = idx + 1;
            if c < counter {
                history.insert(*x, c as u32);
            }
        }
        MemoryGame {
            history,
            counter: counter as u32, 
            last: starting[counter - 1],
        }
    }

    fn turn(&mut self) -> (u32, u32) {
        let prev = self.history.get(&self.last).cloned();
        self.history.insert(self.last, self.counter);
        let age = prev.map(|p| self.counter - p).unwrap_or(0);
        self.counter += 1;
        log::trace!("turn: {} - last: {} prev: {:?} speak: {}", self.counter, self.last, prev, age);
        self.last = age;
        (self.counter, age)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day15");
    const INPUT_EX1: &str = include_str!("../../../input/day15_ex1");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "1009")
    }
    #[test]
    fn verify_p1_ex1() {
        assert_eq!(format!("{}", part1(INPUT_EX1).unwrap()), "436")
    }
    #[test]
    fn verify_p1_examples() {
        assert_eq!(format!("{}", part1("1,3,2").unwrap()), "1");
        assert_eq!(format!("{}", part1("2,1,3").unwrap()), "10");
        assert_eq!(format!("{}", part1("1,2,3").unwrap()), "27");
        assert_eq!(format!("{}", part1("2,3,1").unwrap()), "78");
        assert_eq!(format!("{}", part1("3,2,1").unwrap()), "438");
        assert_eq!(format!("{}", part1("3,1,2").unwrap()), "1836");
    }
    #[test]
    fn verify_p2_ex1() {
        assert_eq!(format!("{}", part2(INPUT_EX1).unwrap()), "175594")
    }
    #[test]
    fn verify_p2_examples() {
        assert_eq!(format!("{}", part2("1,3,2").unwrap()), "2578");
        assert_eq!(format!("{}", part2("2,1,3").unwrap()), "3544142");
        assert_eq!(format!("{}", part2("1,2,3").unwrap()), "261214");
        assert_eq!(format!("{}", part2("2,3,1").unwrap()), "6895259");
        assert_eq!(format!("{}", part2("3,2,1").unwrap()), "18");
        assert_eq!(format!("{}", part2("3,1,2").unwrap()), "362");
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "")
    }
}
