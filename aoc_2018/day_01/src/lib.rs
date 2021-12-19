use std::collections::HashSet;

use anyhow::{anyhow, Result};

pub const YEAR: u32 = 2018;
pub const DAY: u32 = 1;

pub fn part_one(input: &str) -> Result<i32> {
    let mut frequency = 0i32;
    for change in input.trim().lines() {
        match change.parse::<i32>() {
            Ok(change) => frequency += change,
            Err(_) => return Err(anyhow!("invalid frequency change: '{}'", change)),
        }
    }
    Ok(frequency)
}

pub fn part_two(input: &str) -> Result<i32> {
    let mut frequency = 0i32;
    let mut seen = HashSet::new();
    seen.insert(frequency);
    for change in input.trim().lines().cycle() {
        match change.parse::<i32>() {
            Ok(change) => {
                frequency += change;
                if !seen.insert(frequency) {
                    return Ok(frequency);
                }
            }
            Err(_) => return Err(anyhow!("invalid frequency change: '{}'", change)),
        }
    }
    unreachable!();
}

#[test]
fn part_one_example1() -> Result<()> {
    assert_eq!(part_one("+1\n-2\n+3\n+1\n")?, 3);
    Ok(())
}

#[test]
fn part_one_example2() -> Result<()> {
    assert_eq!(part_one("+1\n+1\n+1\n")?, 3);
    Ok(())
}

#[test]
fn part_one_example3() -> Result<()> {
    assert_eq!(part_one("+1\n+1\n-2\n")?, 0);
    Ok(())
}

#[test]
fn part_one_example4() -> Result<()> {
    assert_eq!(part_one("-1\n-2\n-3\n")?, -6);
    Ok(())
}

#[test]
fn part_two_example1() -> Result<()> {
    assert_eq!(part_two("+1\n-2\n+3\n+1\n")?, 2);
    Ok(())
}

#[test]
fn part_two_example2() -> Result<()> {
    assert_eq!(part_two("+1\n-1\n")?, 0);
    Ok(())
}

#[test]
fn part_two_example3() -> Result<()> {
    assert_eq!(part_two("+3\n+3\n+4\n-2\n-4\n")?, 10);
    Ok(())
}

#[test]
fn part_two_example4() -> Result<()> {
    assert_eq!(part_two("-6\n+3\n+8\n+5\n-6\n")?, 5);
    Ok(())
}

#[test]
fn part_two_example5() -> Result<()> {
    assert_eq!(part_two("+7\n+7\n-2\n-7\n-4\n")?, 14);
    Ok(())
}
