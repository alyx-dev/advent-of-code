use anyhow::{Context, Result};

pub const YEAR: u32 = 2017;
pub const DAY: u32 = 1;

pub fn part_one(input: &str) -> Result<u32> {
    let input = input.trim();
    let len = input.len() + 1;
    let mut sum = 0;
    let mut iter = input.chars().cycle().take(len).peekable();
    while let Some(digit) = iter.next() {
        if iter.peek() == Some(&digit) {
            sum += digit.to_digit(10).with_context(|| format!("invalid digit '{}'", digit.escape_default()))?;
        }
    }
    Ok(sum)
}

pub fn part_two(input: &str) -> Result<u32> {
    let input = input.trim();
    let len = input.len();
    let sequence: Vec<_> = input.chars().collect();
    let mut sum = 0;
    for (idx, digit) in sequence.iter().enumerate() {
        let halfway = (idx + len / 2) % len;
        if sequence[halfway] == *digit {
            sum += digit.to_digit(10).with_context(|| format!("invalid digit '{}'", digit.escape_default()))?;
        }
    }
    Ok(sum)
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

#[test]
fn part_two_example1() -> Result<()> {
    assert_eq!(part_two("1212")?, 6);
    Ok(())
}

#[test]
fn part_two_example2() -> Result<()> {
    assert_eq!(part_two("1221")?, 0);
    Ok(())
}

#[test]
fn part_two_example3() -> Result<()> {
    assert_eq!(part_two("123425")?, 4);
    Ok(())
}

#[test]
fn part_two_example4() -> Result<()> {
    assert_eq!(part_two("123123")?, 12);
    Ok(())
}

#[test]
fn part_two_example5() -> Result<()> {
    assert_eq!(part_two("12131415")?, 4);
    Ok(())
}
