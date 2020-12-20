use anyhow::{Context, Result};
use std::fmt;

pub fn part1(input: &str) -> Result<impl fmt::Display> {
    let pb = parse(input)?;
    Ok(pb.iter().filter(|p| p.is_complete()).count())
}
pub fn part2(input: &str) -> Result<impl fmt::Display> {
    let pb = parse(input)?;
    Ok(pb
        .iter()
        .filter(|p| match p.is_valid() {
            Ok(_) => true,
            Err(e) => {
                log::trace!("{:?}\n{}", p, aoc::Error(e));
                false
            }
        })
        .count())
}

#[derive(Debug)]
struct Year(String);
#[derive(Debug)]
struct Color(String);
#[derive(Debug)]
struct Height(String);
#[derive(Debug)]
struct Id(String);

#[derive(Debug, Default)]
struct PassportBuilder {
    byr: Option<Year>,
    iyr: Option<Year>,
    eyr: Option<Year>,
    hgt: Option<Height>,
    hcl: Option<Color>,
    ecl: Option<Color>,
    pid: Option<Id>,
    cid: Option<Id>,
}

impl PassportBuilder {
    fn is_complete(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid(&self) -> Result<()> {
        check_year(&self.byr, 1920, 2002).context("bad birth year")?;
        check_year(&self.iyr, 2010, 2020).context("bad issue year")?;
        check_year(&self.eyr, 2020, 2030).context("bad expiration year")?;
        valid_height(&self.hgt).context("bad height")?;
        valid_hair_color(&self.hcl).context("bad hair color")?;
        valid_eye_color(&self.ecl).context("bad eye color")?;
        valid_passport_id(&self.pid).context("bad passport id")?;
        Ok(())
    }
}

fn check_year(xyr: &Option<Year>, low: u32, high: u32) -> Result<()> {
    let s = if let Some(s) = xyr {
        s.0.as_str()
    } else {
        anyhow::bail!("no year")
    };

    let y = s.parse::<u32>().context("year not number")?;
    if y < low {
        anyhow::bail!("year too low")
    } else if y > high {
        anyhow::bail!("year too high")
    }
    Ok(())
}

fn valid_height(hgt: &Option<Height>) -> Result<()> {
    let s = if let Some(s) = hgt {
        s.0.as_str()
    } else {
        anyhow::bail!("no height")
    };

    let value = s[..s.len() - 2]
        .parse::<u32>()
        .context("height did not contain numeric value")?;
    match &s[s.len() - 2..] {
        "cm" => {
            if value < 150 || value > 193 {
                anyhow::bail!("invalid hight in cm: {}", value);
            }
        }
        "in" => {
            if value < 59 || value > 76 {
                anyhow::bail!("invalid hight in in: {}", value);
            }
        }
        unit => {
            anyhow::bail!("height contained invalid unit: {}", unit);
        }
    }

    Ok(())
}
fn valid_hair_color(hcl: &Option<Color>) -> Result<()> {
    let s = if let Some(s) = hcl {
        s.0.as_str()
    } else {
        anyhow::bail!("no color")
    };

    for (idx, c) in s.chars().enumerate() {
        if idx == 0 {
            if c != '#' {
                anyhow::bail!("did not start with '#'")
            }
        } else {
            match c {
                '0'..='9' | 'a'..='f' => {}
                _ => anyhow::bail!("invalid char #{} {:?}", idx, c),
            }
        }
    }
    Ok(())
}
fn valid_eye_color(ecl: &Option<Color>) -> Result<()> {
    let s = if let Some(s) = ecl {
        s.0.as_str()
    } else {
        anyhow::bail!("no color")
    };

    match s {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
        _ => anyhow::bail!("invalid eye color: {:?}", s),
    }

    Ok(())
}

fn valid_passport_id(pid: &Option<Id>) -> Result<()> {
    let s = if let Some(s) = pid {
        s.0.as_str()
    } else {
        anyhow::bail!("no passport id")
    };

    if s.len() != 9 {
        anyhow::bail!("incorrect pid length")
    }

    for c in s.chars() {
        match c {
            '0'..='9' => {}
            _ => anyhow::bail!("char {:?} was not a number", c),
        }
    }

    Ok(())
}

fn parse(input: &str) -> Result<Vec<PassportBuilder>> {
    let mut passports = Vec::new();
    let mut builder = PassportBuilder::default();

    for line in input.lines() {
        if line.is_empty() {
            let mut next = PassportBuilder::default();
            std::mem::swap(&mut next, &mut builder);
            passports.push(next);
        } else {
            parse_kv(line, &mut builder).with_context(|| format!("failed to parse: {:?}", line))?;
        }
    }
    passports.push(builder);
    Ok(passports)
}

fn parse_kv(s: &str, builder: &mut PassportBuilder) -> Result<()> {
    for pair in s.split_ascii_whitespace() {
        let mut split = pair.splitn(2, ':').into_iter();
        let k = split.next().ok_or_else(|| anyhow::anyhow!("kv is empty"))?;
        let v = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("kv did not have ':'"))?;
        match k {
            "byr" => builder.byr = Some(Year(v.to_string())),
            "iyr" => builder.iyr = Some(Year(v.to_string())),
            "eyr" => builder.eyr = Some(Year(v.to_string())),
            "hgt" => builder.hgt = Some(Height(v.to_string())),
            "hcl" => builder.hcl = Some(Color(v.to_string())),
            "ecl" => builder.ecl = Some(Color(v.to_string())),
            "pid" => builder.pid = Some(Id(v.to_string())),
            "cid" => builder.cid = Some(Id(v.to_string())),
            _ => anyhow::bail!("unknown key: {:?}", k),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day4");
    const EX: &str = include_str!("../../../input/day4_ex");
    const EX_INVALID: &str = include_str!("../../../input/day4_ex_invalid");
    const EX_VALID: &str = include_str!("../../../input/day4_ex_valid");

    #[test]
    fn verify_p1() {
        assert_eq!(format!("{}", part1(INPUT).unwrap()), "202")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(format!("{}", part2(INPUT).unwrap()), "137")
    }
    #[test]
    fn p1_ex() {
        assert_eq!(format!("{}", part1(EX).unwrap()), "2")
    }
    #[test]
    fn p2_ex_valid() {
        assert_eq!(format!("{}", part2(EX_VALID).unwrap()), "4")
    }
    #[test]
    fn p2_ex_invalid() {
        assert_eq!(format!("{}", part2(EX_INVALID).unwrap()), "0")
    }
}
