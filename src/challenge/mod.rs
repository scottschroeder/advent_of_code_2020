use anyhow::{anyhow as ah, Context, Result};
use std::{fs, io::Read, path};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;
mod day12;

fn f<T: std::fmt::Display>(t: Result<T>) -> Result<()> {
    t.map(|t| {
        println!("{}", t);
    })
}

pub fn run(args: &clap::ArgMatches) -> Result<()> {
    let day = args.value_of("day").unwrap().parse::<u32>()?;
    let part = args.value_of("part").unwrap().parse::<u32>()?;
    let input = read_to_string(args.value_of("input").unwrap())?;
    log::debug!("running day {}:{}", day, part);
    match (day, part) {
        (1, 1) => f(day1::part1(&input)),
        (1, 2) => f(day1::part2(&input)),
        (2, 1) => f(day2::part1(&input)),
        (2, 2) => f(day2::part2(&input)),
        (3, 1) => f(day3::part1(&input)),
        (3, 2) => f(day3::part2(&input)),
        (4, 1) => f(day4::part1(&input)),
        (4, 2) => f(day4::part2(&input)),
        (5, 1) => f(day5::part1(&input)),
        (5, 2) => f(day5::part2(&input)),
        (6, 1) => f(day6::part1(&input)),
        (6, 2) => f(day6::part2(&input)),
        (7, 1) => f(day7::part1(&input)),
        (7, 2) => f(day7::part2(&input)),
        (8, 1) => f(day8::part1(&input)),
        (8, 2) => f(day8::part2(&input)),
        (9, 1) => f(day9::part1(&input)),
        (9, 2) => f(day9::part2(&input)),
        (10, 1) => f(day10::part1(&input)),
        (10, 2) => f(day10::part2(&input)),
        (11, 1) => f(day11::part1(&input)),
        (11, 2) => f(day11::part2(&input)),
        (12, 1) => f(day12::part1(&input)),
        (12, 2) => f(day12::part2(&input)),
        (d, p) => Err(ah!("unimplemented challenge day {} part {}", d, p)),
    }
}

fn read_to_string<P: AsRef<path::Path>>(path: P) -> Result<String> {
    log::trace!("Reading content of file: {}", path.as_ref().display());
    let mut f = fs::File::open(&path)
        .with_context(|| format!("Unable to open path: {}", path.as_ref().display()))?;

    let mut result = String::new();

    f.read_to_string(&mut result)?;
    Ok(result)
}
