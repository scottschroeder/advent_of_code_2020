use anyhow::Result;
use std::fmt;
mod memory_sequence;

const PT1_SPOKEN: usize = 2020;
const PT2_SPOKEN: usize = 30000000;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let starting = parse_input(input)?;
    Ok(number_at(&starting, PT1_SPOKEN))
}

pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let starting = parse_input(input)?;
    Ok(number_at(&starting, PT2_SPOKEN))
}

fn number_at(starting: &[u32], target: usize) -> usize {
    let mut game = memory_sequence::Sequence::init(starting, target);
    game.nth(target - starting.len() - 1).unwrap()
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
    fn verify_p1_ex1_sequence() {
        let starting = parse_input(INPUT_EX1).unwrap();
        let target = 10 - starting.len();
        let game = memory_sequence::Sequence::init(&starting, target);
        let seq = game.take(target).collect::<Vec<_>>();
        assert_eq!(seq, vec![0, 3, 3, 1, 0, 4, 0])
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
    fn verify_p2_example_1() {
        assert_eq!(format!("{}", part2("1,3,2").unwrap()), "2578");
    }
    #[test]
    fn verify_p2_example_2() {
        assert_eq!(format!("{}", part2("2,1,3").unwrap()), "3544142");
    }
    #[test]
    fn verify_p2_example_3() {
        assert_eq!(format!("{}", part2("1,2,3").unwrap()), "261214");
    }
    #[test]
    fn verify_p2_example_4() {
        assert_eq!(format!("{}", part2("2,3,1").unwrap()), "6895259");
    }
    #[test]
    fn verify_p2_example_5() {
        assert_eq!(format!("{}", part2("3,2,1").unwrap()), "18");
    }
    #[test]
    fn verify_p2_example_6() {
        assert_eq!(format!("{}", part2("3,1,2").unwrap()), "362");
    }

    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "62714")
    }
}
