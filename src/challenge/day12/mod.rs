use anyhow::{Context, Result};
use aoc::{grid::compass::Direction, Point};
use std::fmt;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let instrs = parse(input)?;
    let mut ship = Ship::default();
    for i in instrs {
        ship.step(i);
        log::trace!("{:?} {:?}", i, ship);
    }
    Ok(manhattan_distance(ship.position))
}
pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let instrs = parse(input)?;
    let mut ship = WayPointShip::default();
    for i in instrs {
        ship.step(i);
        log::trace!("{:?} {:?}", i, ship);
    }
    Ok(manhattan_distance(ship.postion))
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Instruction {
    command: Command,
    magnitude: i64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Command {
    North,
    South,
    East,
    West,
    Forward,
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct WayPointShip {
    postion: Point<i64>,
    waypoint: Point<i64>,
}

impl Default for WayPointShip {
    fn default() -> Self {
        WayPointShip {
            postion: Point::new(0, 0),
            waypoint: Point::new(10, 1),
        }
    }
}

const Q_TRIG: [i64; 4] = [0, 1, 0, -1];

#[inline]
fn q_sin(q: usize) -> i64 {
    Q_TRIG[q % 4]
}
#[inline]
fn q_cos(q: usize) -> i64 {
    Q_TRIG[(q + 1) % 4]
}

fn rotate(p: Point<i64>, deg: i64) -> Point<i64> {
    let q = ((deg + 360) / 90) as usize;
    Point::new(
        p.x * q_cos(q) - p.y * q_sin(q),
        p.x * q_sin(q) + p.y * q_cos(q),
    )
}

impl WayPointShip {
    fn step(&mut self, instr: Instruction) {
        let translate = |dir: Direction| self.waypoint + dir.delta().scale(instr.magnitude);

        match instr.command {
            Command::North => self.waypoint = translate(Direction::North),
            Command::South => self.waypoint = translate(Direction::South),
            Command::East => self.waypoint = translate(Direction::East),
            Command::West => self.waypoint = translate(Direction::West),
            Command::Right => self.waypoint = rotate(self.waypoint, -instr.magnitude),
            Command::Left => self.waypoint = rotate(self.waypoint, instr.magnitude),
            Command::Forward => {
                self.postion = self.postion + self.waypoint.scale(instr.magnitude);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ship {
    heading: Direction,
    position: Point<i64>,
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            heading: Direction::East,
            position: Point::new(0, 0),
        }
    }
}

fn manhattan_distance(p: Point<i64>) -> i64 {
    p.x.abs() + p.y.abs()
}

impl Ship {
    fn step(&mut self, instr: Instruction) {
        let translate = |dir: Direction| {
            let dir = dir.delta();
            self.position + dir.scale(instr.magnitude)
        };
        let rotate = |clockwise: bool| {
            let iter = instr.magnitude / 90;
            let mut r = self.heading;
            for _ in 0..iter {
                r = if clockwise {
                    r.clockwise()
                } else {
                    r.anticlockwise()
                };
            }
            r
        };
        match instr.command {
            Command::North => self.position = translate(Direction::North),
            Command::South => self.position = translate(Direction::South),
            Command::East => self.position = translate(Direction::East),
            Command::West => self.position = translate(Direction::West),
            Command::Forward => self.position = translate(self.heading),
            Command::Right => self.heading = rotate(true),
            Command::Left => self.heading = rotate(false),
        }
    }
}

fn parse(input: &str) -> Result<Vec<Instruction>> {
    input.lines().map(|l| parse_instr(l)).collect()
}

fn parse_instr(s: &str) -> Result<Instruction> {
    let command = match &s[0..1] {
        "N" => Command::North,
        "S" => Command::South,
        "E" => Command::East,
        "W" => Command::West,
        "F" => Command::Forward,
        "R" => Command::Right,
        "L" => Command::Left,
        c => anyhow::bail!("unknown cmd: {:?}", c),
    };
    let numeric = &s[1..];
    let magnitude = numeric
        .parse::<i64>()
        .with_context(|| format!("could not parse number: {:?}", numeric))?;
    Ok(Instruction { command, magnitude })
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day12");
    const EX: &str = include_str!("../../../input/day12_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "882")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "28885")
    }
    #[test]
    fn p1_ex1() {
        assert_eq!(format!("{}", part1(EX).unwrap()), "25")
    }
    #[test]
    fn p2_ex1() {
        assert_eq!(format!("{}", part2(EX).unwrap()), "286")
    }
}
