use anyhow::{anyhow, Result};

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 1;

pub fn part_one(input: &str) -> Result<i32> {
    let mut floor = 0;
    for instruction in input.chars() {
        match instruction {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => return Err(anyhow!("invalid instruction: {}", instruction.escape_default())),
        }
    }
    Ok(floor)
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut floor = 0;
    for (position, instruction) in input.char_indices() {
        match instruction {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => return Err(anyhow!("invalid instruction: {}", instruction.escape_default())),
        }
        if floor == -1 {
            return Ok(position + 1);
        }
    }
    Err(anyhow!("basement not found"))
}

#[test]
fn part_one_example1() -> Result<()> {
    assert_eq!(part_one("(())")?, 0);
    assert_eq!(part_one("()()")?, 0);
    Ok(())
}

#[test]
fn part_one_example2() -> Result<()> {
    assert_eq!(part_one("(((")?, 3);
    assert_eq!(part_one("(()(()(")?, 3);
    Ok(())
}

#[test]
fn part_one_example3() -> Result<()> {
    assert_eq!(part_one("))(((((")?, 3);
    Ok(())
}

#[test]
fn part_one_example4() -> Result<()> {
    assert_eq!(part_one("())")?, -1);
    assert_eq!(part_one("))(")?, -1);
    Ok(())
}

#[test]
fn part_one_example5() -> Result<()> {
    assert_eq!(part_one(")))")?, -3);
    assert_eq!(part_one(")())())")?, -3);
    Ok(())
}

#[test]
fn part_two_example1() -> Result<()> {
    assert_eq!(part_two(")")?, 1);
    Ok(())
}

#[test]
fn part_two_example2() -> Result<()> {
    assert_eq!(part_two("()())")?, 5);
    Ok(())
}
