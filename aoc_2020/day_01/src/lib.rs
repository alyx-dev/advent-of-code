use std::collections::HashSet;

use anyhow::{anyhow, Context, Result};

pub const YEAR: u32 = 2020;
pub const DAY: u32 = 1;

pub fn part_one(input: &str) -> Result<u32> {
    let mut report = HashSet::new();
    for expense in input.trim().lines() {
        let expense = expense.parse::<u32>().with_context(|| format!("invalid expense: '{}", expense))?;
        report.insert(expense);
    }
    for x in report.iter() {
        if let Some(y) = report.get(&(2020 - x)) {
            return Ok(x * y);
        }
    }
    Err(anyhow!("complement expenses not found"))
}

pub fn part_two(input: &str) -> Result<u32> {
    let mut report = HashSet::new();
    for expense in input.trim().lines() {
        let expense = expense.parse::<u32>().with_context(|| format!("invalid expense: '{}", expense))?;
        report.insert(expense);
    }
    for x in report.iter() {
        for y in report.iter() {
            if let Some(z) = report.get(&(2020 - x - y)) {
                return Ok(x * y * z);
            }
        }
    }
    Err(anyhow!("complement expenses not found"))
}

#[test]
fn part_one_example() -> Result<()> {
    assert_eq!(part_one("1721\n979\n366\n299\n675\n1456\n")?, 514579);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    assert_eq!(part_two("1721\n979\n366\n299\n675\n1456\n")?, 241861950);
    Ok(())
}
