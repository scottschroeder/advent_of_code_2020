use self::rule_graph::{Bag, Rules};
use anyhow::{anyhow as ah, Context, Result};
use std::fmt;

mod rule_graph;

const MY_BAG: &str = "shiny gold";

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let rules = create_rule_tree(input)?;
    Ok(rules.parents_of(MY_BAG))
}
pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let rules = create_rule_tree(input)?;
    Ok(rules.total_bags(MY_BAG))
}

fn create_rule_tree<'a>(input: &'a str) -> Result<Rules<'a>> {
    let mut rules = Rules::default();
    for l in input.lines() {
        add_rule(&mut rules, l)?;
    }
    Ok(rules)
}

fn add_rule<'a>(rules: &mut Rules<'a>, input: &'a str) -> Result<()> {
    let mut rule_contains = input.splitn(2, " contain ").into_iter();
    let src = rule_contains.next().ok_or_else(|| ah!("rule was empty"))?;
    let dst = rule_contains
        .next()
        .ok_or_else(|| ah!("rule did not have 'contain'"))?;
    let bag = parse_bag(src).with_context(|| format!("{:?} was not a bag", src))?;
    add_rule_targets(rules, bag, dst).with_context(|| format!("failed to apply rule: {:?}", input))
}

fn parse_bag<'a>(s: &'a str) -> Result<Bag<'a>> {
    let bag = s
        .find("bag")
        .ok_or_else(|| ah!("bag did not contain word 'bag'"))?;
    let b = &s[..bag];
    Ok(Bag(b.trim()))
}

fn parse_numerical_bags<'a>(s: &'a str) -> Result<(usize, Bag<'a>)> {
    let s = s.trim();
    let mut rule_numeric = s.splitn(2, " ").into_iter();
    let num = rule_numeric
        .next()
        .ok_or_else(|| ah!("numerical bags empty"))?;
    let count = num
        .parse::<usize>()
        .with_context(|| format!("{:?} was not a number", num))?;
    let bag_text = rule_numeric
        .next()
        .ok_or_else(|| ah!("numerical bags could not be split"))?;
    let bag = parse_bag(bag_text).with_context(|| format!("{:?} was not a bag", bag_text))?;
    Ok((count, bag))
}

fn add_rule_targets<'a>(rules: &mut Rules<'a>, bag: Bag<'a>, targets: &'a str) -> Result<()> {
    if targets == "no other bags." {
        rules.terminal(bag);
    } else {
        for target in targets.split(',').into_iter() {
            let (c, b) = parse_numerical_bags(target)
                .with_context(|| format!("failed to parse target {:?}", target))?;
            rules.insert(bag, b, c);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day7");
    const EX1: &str = include_str!("../../../input/day7_ex");
    const EX2: &str = include_str!("../../../input/day7_ex2");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "235")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "158493")
    }

    #[test]
    fn pt1_ex1() {
        assert_eq!(format!("{}", part1(EX1).unwrap()), "4")
    }

    #[test]
    fn pt2_ex1() {
        assert_eq!(format!("{}", part2(EX1).unwrap()), "32")
    }
    #[test]
    fn pt2_ex2() {
        assert_eq!(format!("{}", part2(EX2).unwrap()), "126")
    }
}
