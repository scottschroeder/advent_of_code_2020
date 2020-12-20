use anyhow::Result;
use aoc::{
    grid::{fixed_grid::FixedGrid, grid_types::GridHeight, repeat_grid::HorizontalRepeat},
    Point,
};
use std::fmt;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let trees = parse(input)?;
    Ok(trees_hit(&trees, Point::new(0, 0), Point::new(3, 1)))
}
pub fn part2(input: &str) -> Result<impl fmt::Display> {
    Ok("")
}

struct Trees {
    inner: HorizontalRepeat<FixedGrid<bool>>,
}

fn trees_hit(trees: &Trees, start: Point, slope: Point) -> usize {
    (0..)
        .map(|idx| start + Point::new(slope.x * idx, slope.y * idx))
        .take_while(|p| p.y < trees.inner.height())
        .filter(|p| trees.inner[*p])
        .count()
}

fn parse(input: &str) -> Result<Trees> {
    let inner = FixedGrid::parse_ascii_grid(input, |c| Ok(c == '#'))?;
    Ok(Trees {
        inner: HorizontalRepeat::new(inner),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day3");
    const EX: &str = include_str!("../../../input/day3_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "278")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "")
    }
    #[test]
    fn day1_ex() {
        assert_eq!(format!("{}", part1(EX).unwrap()), "7")
    }
}
