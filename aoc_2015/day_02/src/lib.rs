use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 2;

pub fn part_one(input: &str) -> Result<u32> {
    let mut paper = 0;
    for line in input.trim().lines() {
        let present: Present = line.parse()?;
        paper += present.paper();
    }
    Ok(paper)
}

pub fn part_two(input: &str) -> Result<u32> {
    let mut ribbon = 0;
    for line in input.trim().lines() {
        let present: Present = line.parse()?;
        ribbon += present.ribbon();
    }
    Ok(ribbon)
}

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn paper(&self) -> u32 {
        let surfaces = [self.length * self.width, self.width * self.height, self.height * self.length];
        surfaces.iter().sum::<u32>() * 2 + surfaces.iter().min().unwrap()
    }

    fn ribbon(&self) -> u32 {
        let perimeters = [2 * (self.length + self.width), 2 * (self.width + self.height), 2 * (self.height + self.length)];
        perimeters.iter().min().unwrap() + self.length * self.width * self.height
    }
}

impl FromStr for Present {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split('x');
        let length = match split.next().map(|x| x.parse()) {
            Some(Ok(length)) => length,
            _ => return Err(anyhow!("invalid present dimensions '{}'", s.trim())),
        };
        let width = match split.next().map(|x| x.parse()) {
            Some(Ok(width)) => width,
            _ => return Err(anyhow!("invalid present dimensions '{}'", s.trim())),
        };
        let height = match split.next().map(|x| x.parse()) {
            Some(Ok(height)) => height,
            _ => return Err(anyhow!("invalid present dimensions '{}'", s.trim())),
        };
        Ok(Present { length, width, height })
    }
}

#[test]
fn part_one_example1() -> Result<()> {
    assert_eq!("2x3x4".parse::<Present>()?.paper(), 58);
    Ok(())
}

#[test]
fn part_one_example2() -> Result<()> {
    assert_eq!("1x1x10".parse::<Present>()?.paper(), 43);
    Ok(())
}

#[test]
fn part_two_example1() -> Result<()> {
    assert_eq!("2x3x4".parse::<Present>()?.ribbon(), 34);
    Ok(())
}

#[test]
fn part_two_example2() -> Result<()> {
    assert_eq!("1x1x10".parse::<Present>()?.ribbon(), 14);
    Ok(())
}
