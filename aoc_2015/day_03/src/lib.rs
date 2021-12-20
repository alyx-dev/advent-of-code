use std::collections::HashSet;

use anyhow::{anyhow, Result};

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 3;

pub fn part_one(input: &str) -> Result<usize> {
    let mut houses = HashSet::new();
    let mut santa = Location::default();
    houses.insert(santa);
    for step in input.chars() {
        santa.step(step)?;
        houses.insert(santa);
    }
    Ok(houses.len())
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut houses = HashSet::new();
    houses.insert(Location::default());
    let mut santa = Location::default();
    let mut robot = Location::default();
    for step in input.chars().step_by(2) {
        santa.step(step)?;
        houses.insert(santa);
    }
    for step in input.chars().skip(1).step_by(2) {
        robot.step(step)?;
        houses.insert(robot);
    }
    Ok(houses.len())
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Location(i32, i32);

impl Location {
    fn step(&mut self, step: char) -> Result<()> {
        match step {
            '>' => self.0 += 1,
            '<' => self.0 -= 1,
            '^' => self.1 += 1,
            'v' => self.1 -= 1,
            _ => return Err(anyhow!("invalid move: '{}'", step.escape_default())),
        }
        Ok(())
    }
}

#[test]
fn part_one_example1() -> Result<()> {
    assert_eq!(part_one(">")?, 2);
    Ok(())
}

#[test]
fn part_one_example2() -> Result<()> {
    assert_eq!(part_one("^>v<")?, 4);
    Ok(())
}

#[test]
fn part_one_example3() -> Result<()> {
    assert_eq!(part_one("^v^v^v^v^v")?, 2);
    Ok(())
}

#[test]
fn part_two_example1() -> Result<()> {
    assert_eq!(part_two("^v")?, 3);
    Ok(())
}

#[test]
fn part_two_example2() -> Result<()> {
    assert_eq!(part_two("^>v<")?, 3);
    Ok(())
}

#[test]
fn part_two_example3() -> Result<()> {
    assert_eq!(part_two("^v^v^v^v^v")?, 11);
    Ok(())
}
