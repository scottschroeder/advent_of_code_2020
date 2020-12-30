use anyhow::Result;
use aoc::{grid::fixed_grid::FixedGrid, Point};
use std::fmt;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let mut b = parse(input)?;
    while b.step(4, check_immediate_occupied) {
        log::trace!("\n{:?}", b);
    }
    log::trace!("\n{:?}", b);
    Ok(b.count())
}
pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let mut b = parse(input)?;
    while b.step(5, check_sight_occupied) {
        log::trace!("\n{:?}", b);
    }
    log::trace!("\n{:?}", b);
    Ok(b.count())
}

struct Board {
    a: FixedGrid<Tile>,
    b: FixedGrid<Tile>,
    swp: bool,
}

fn check_immediate_occupied(center: Point<i64>, g: &FixedGrid<Tile>) -> usize {
    center
        .adjacent_all()
        .filter(|p| {
            g.maybe_point_to_idx(*p)
                .map(|idx| g.as_slice()[idx] == Tile::Occupied)
                .unwrap_or(false)
        })
        .count()
}

fn check_sight_occupied(center: Point<i64>, g: &FixedGrid<Tile>) -> usize {
    Point::new(0, 0)
        .adjacent_all()
        .filter(|direction| scan_direction(center, *direction, g))
        .count()
}

fn scan_direction(center: Point<i64>, direction: Point<i64>, g: &FixedGrid<Tile>) -> bool {
    let mut mag = 1;
    loop {
        let p = center + Point::new(direction.x * mag, direction.y * mag);
        if let Some(idx) = g.maybe_point_to_idx(p) {
            match g.as_slice()[idx] {
                Tile::Floor => mag += 1,
                Tile::Empty => return false,
                Tile::Occupied => return true,
            }
        } else {
            return false;
        }
    }
}

impl Board {
    fn inner(&self) -> &FixedGrid<Tile> {
        if self.swp {
            &self.b
        } else {
            &self.a
        }
    }
    fn count(&self) -> usize {
        self.inner()
            .raw_iter()
            .filter(|t| **t == Tile::Occupied)
            .count()
    }
    fn step<F: Fn(Point<i64>, &FixedGrid<Tile>) -> usize>(
        &mut self,
        tolerance: usize,
        check_point: F,
    ) -> bool {
        let Board { a, b, swp } = self;

        let (read, write) = if *swp { (b, a) } else { (a, b) };

        read.raw_iter()
            .zip(write.mut_iter())
            .enumerate()
            .for_each(|(idx, (t, t_write))| {
                let p = read.idx_to_point(idx);
                *t_write = match t {
                    Tile::Floor => Tile::Floor,
                    Tile::Empty => {
                        if check_point(p, &read) == 0 {
                            Tile::Occupied
                        } else {
                            Tile::Empty
                        }
                    }
                    Tile::Occupied => {
                        if check_point(p, &read) >= tolerance {
                            Tile::Empty
                        } else {
                            Tile::Occupied
                        }
                    }
                }
            });
        *swp = !*swp;
        read != write
    }
}

fn parse(input: &str) -> Result<Board> {
    let grid = FixedGrid::parse_ascii_grid(input, |c| match c {
        '.' => Ok(Tile::Floor),
        'L' => Ok(Tile::Empty),
        '#' => Ok(Tile::Occupied),
        _ => Err(anyhow::anyhow!("unknown tile char: {:?}", c)),
    })?;
    Ok(Board {
        a: grid.clone(),
        b: grid,
        swp: false,
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::Floor => ".",
            Tile::Empty => "L",
            Tile::Occupied => "#",
        };
        write!(f, "{}", c)
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day11");
    const EX: &str = include_str!("../../../input/day11_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "2412")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "2176")
    }
    #[test]
    fn check_p1ex1() {
        assert_eq!(format!("{}", part1(EX).unwrap()), "37")
    }
    #[test]
    fn check_p2ex1() {
        assert_eq!(format!("{}", part2(EX).unwrap()), "26")
    }
}
