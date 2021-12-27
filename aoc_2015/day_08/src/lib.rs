use anyhow::{anyhow, Context, Result};

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 8;

pub fn part_one(input: &str) -> Result<usize> {
    let mut length = 0;
    let mut size = 0;
    for line in input.lines() {
        length += string_length(line.trim())?;
        size += string_size(line.trim())?;
    }
    Ok(length - size)
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut length = 0;
    let mut encoded = 0;
    for line in input.lines() {
        length += string_length(line.trim())?;
        encoded += string_encode_length(line.trim())?;
    }
    Ok(encoded - length)
}

fn string_length(str: &str) -> Result<usize> {
    Ok(str.len())
}

fn string_size(str: &str) -> Result<usize> {
    let mut size = 0;
    let str = str.strip_prefix('"').with_context(|| format!("invalid string: '{}'", str))?;
    let str = str.strip_suffix('"').with_context(|| format!("invalid string: '{}'", str))?;
    let mut iter = str.bytes().peekable();
    while let Some(next) = iter.next() {
        if next == b'\\' {
            match iter.peek() {
                Some(b'\\' | b'"') => {
                    iter.next();
                }
                Some(b'x') => {
                    iter.nth(2);
                }
                _ => return Err(anyhow!("invalid string: '{}'", str)),
            }
        }
        size += 1;
    }
    Ok(size)
}

fn string_encode_length(str: &str) -> Result<usize> {
    let mut length = 2;
    for byte in str.bytes() {
        match byte {
            b'\\' | b'"' => length += 2,
            _ => length += 1,
        }
    }
    Ok(length)
}

#[test]
fn part_one_example1() -> Result<()> {
    assert_eq!(string_length(r#""""#)?, 2);
    assert_eq!(string_size(r#""""#)?, 0);
    Ok(())
}

#[test]
fn part_one_example2() -> Result<()> {
    assert_eq!(string_length(r#""abc""#)?, 5);
    assert_eq!(string_size(r#""abc""#)?, 3);
    Ok(())
}

#[test]
fn part_one_example3() -> Result<()> {
    assert_eq!(string_length(r#""aaa\"aaa""#)?, 10);
    assert_eq!(string_size(r#""aaa\"aaa""#)?, 7);
    Ok(())
}

#[test]
fn part_one_example4() -> Result<()> {
    assert_eq!(string_length(r#""\x27""#)?, 6);
    assert_eq!(string_size(r#""\x27""#)?, 1);
    Ok(())
}

#[test]
fn part_two_example1() -> Result<()> {
    assert_eq!(string_length(r#""""#)?, 2);
    assert_eq!(string_encode_length(r#""""#)?, 6);
    Ok(())
}

#[test]
fn part_two_example2() -> Result<()> {
    assert_eq!(string_length(r#""abc""#)?, 5);
    assert_eq!(string_encode_length(r#""abc""#)?, 9);
    Ok(())
}

#[test]
fn part_two_example3() -> Result<()> {
    assert_eq!(string_length(r#""aaa\"aaa""#)?, 10);
    assert_eq!(string_encode_length(r#""aaa\"aaa""#)?, 16);
    Ok(())
}

#[test]
fn part_two_example4() -> Result<()> {
    assert_eq!(string_length(r#""\x27""#)?, 6);
    assert_eq!(string_encode_length(r#""\x27""#)?, 11);
    Ok(())
}
