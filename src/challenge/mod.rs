use anyhow::{anyhow as ah, Context, Result};
use std::{fs, io::Read, path};

mod day1;

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
