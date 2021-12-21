use std::collections::HashSet;

use anyhow::Result;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 5;

pub fn part_one(input: &str) -> Result<u32> {
    let mut nice = 0;
    for line in input.trim().lines() {
        if is_nice_one(line) {
            nice += 1;
        }
    }
    Ok(nice)
}

pub fn part_two(input: &str) -> Result<u32> {
    let mut nice = 0;
    for line in input.trim().lines() {
        if is_nice_two(line) {
            nice += 1;
        }
    }
    Ok(nice)
}

fn is_nice_one(string: &str) -> bool {
    if string.chars().filter(|c| "aeiou".contains(*c)).count() < 3 {
        return false;
    }
    let mut iter = string.chars().peekable();
    while let Some(char) = iter.next() {
        match iter.peek() {
            Some(peek) if *peek == char => break,
            Some(_) => continue,
            None => return false,
        }
    }
    if string.contains("ab") || string.contains("cd") || string.contains("pq") || string.contains("xy") {
        return false;
    }
    true
}

fn is_nice_two(string: &str) -> bool {
    let mut set = HashSet::new();
    let mut iter = string.as_bytes().windows(2).peekable();
    while let Some(mut next) = iter.next() {
        if iter.peek() == Some(&next) {
            next = match iter.next() {
                Some(next) => next,
                None => return false,
            }
        }
        if !set.insert(next) {
            break;
        }
        if iter.peek().is_none() {
            return false;
        }
    }
    if !string.as_bytes().windows(3).any(|w| w[0] == w[2]) {
        return false;
    }
    true
}

#[test]
fn part_one_example1() {
    assert!(is_nice_one("ugknbfddgicrmopn"));
}

#[test]
fn part_one_example2() {
    assert!(is_nice_one("aaa"));
}

#[test]
fn part_one_example3() {
    assert!(!is_nice_one("jchzalrnumimnmhp"));
}

#[test]
fn part_one_example4() {
    assert!(!is_nice_one("haegwjzuvuyypxyu"));
}

#[test]
fn part_one_example5() {
    assert!(!is_nice_one("dvszwmarrgswjxmb"));
}

#[test]
fn part_two_example1() {
    assert!(is_nice_two("qjhvhtzxzqqjkmpb"));
}

#[test]
fn part_two_example2() {
    assert!(is_nice_two("xxyxx"));
}

#[test]
fn part_two_example3() {
    assert!(!is_nice_two("uurcxstgmygtbstg"));
}

#[test]
fn part_two_example4() {
    assert!(!is_nice_two("ieodomkazucvgmuy"));
}

#[test]
fn part_two_example5() {
    assert!(!is_nice_two("xxx"));
}

#[test]
fn part_two_example6() {
    assert!(is_nice_two("xxxx"));
}
