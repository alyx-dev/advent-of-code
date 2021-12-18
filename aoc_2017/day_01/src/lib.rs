use anyhow::{Context, Result};

pub const YEAR: u32 = 2017;
pub const DAY: u32 = 1;

pub fn part_one(input: &str) -> Result<u32> {
    let input = input.trim();
    let len = input.len() + 1;
    let mut sum = 0;
    let mut iter = input.chars().cycle().take(len).peekable();
    while let Some(digit) = iter.next() {
        match iter.peek() {
            Some(x) if *x == digit => {
                sum += x
                    .to_digit(10)
                    .with_context(|| format!("invalid digit '{}'", x.escape_default()))?;
            }
            _ => continue,
        }
    }
    Ok(sum)
}

pub fn part_two(_: &str) -> Result<u32> {
    Ok(2)
}

#[test]
fn part_one_example1() -> Result<()> {
    assert_eq!(part_one("1122")?, 3);
    Ok(())
}

#[test]
fn part_one_example2() -> Result<()> {
    assert_eq!(part_one("1111")?, 4);
    Ok(())
}

#[test]
fn part_one_example3() -> Result<()> {
    assert_eq!(part_one("1234")?, 0);
    Ok(())
}

#[test]
fn part_one_example4() -> Result<()> {
    assert_eq!(part_one("91212129")?, 9);
    Ok(())
}
