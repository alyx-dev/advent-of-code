use anyhow::{Context, Result};

pub const YEAR: u32 = 2021;
pub const DAY: u32 = 1;

pub fn part_one(input: &str) -> Result<u32> {
    let mut report = Vec::new();
    for line in input.trim().lines() {
        let sweep = line.parse::<u32>().with_context(|| format!("invalid sonar sweep: '{}'", line))?;
        report.push(sweep);
    }
    let mut increase = 0;
    let mut iter = report.into_iter().peekable();
    while let Some(sweep) = iter.next() {
        match iter.peek() {
            Some(peek) if *peek > sweep => increase += 1,
            None => break,
            _ => continue,
        }
    }
    Ok(increase)
}

pub fn part_two(input: &str) -> Result<u32> {
    let mut report = Vec::new();
    for line in input.trim().lines() {
        let sweep = line.parse::<u32>().with_context(|| format!("invalid sonar sweep: '{}'", line))?;
        report.push(sweep);
    }
    let mut increase = 0;
    let mut iter = report.windows(3).peekable();
    while let Some(window) = iter.next() {
        match iter.peek() {
            Some(peek) if peek.iter().sum::<u32>() > window.iter().sum() => increase += 1,
            None => break,
            _ => continue,
        };
    }
    Ok(increase)
}

#[test]
fn part_one_example() -> Result<()> {
    assert_eq!(part_one("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n")?, 7);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    assert_eq!(part_two("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n")?, 5);
    Ok(())
}
