use std::collections::HashSet;

use anyhow::{anyhow, Result};

pub const YEAR: u32 = 2016;
pub const DAY: u32 = 1;

pub fn part_one(input: &str) -> Result<i32> {
    let mut position = Position::default();
    for instruction in input.trim().split(',').map(|str| str.trim()) {
        match instruction.chars().next() {
            Some('L') => position.turn_left(),
            Some('R') => position.turn_right(),
            _ => return Err(anyhow!("invalid instruction: '{}'", instruction)),
        }
        let blocks = match instruction.get(1..).map(|str| str.parse()) {
            Some(Ok(x)) => x,
            _ => return Err(anyhow!("invalid instruction: '{}'", instruction)),
        };
        position.walk(blocks);
    }
    Ok(position.distance())
}

pub fn part_two(input: &str) -> Result<i32> {
    let mut position = Position::default();
    let mut visited = HashSet::new();
    for instruction in input.trim().split(',').map(|str| str.trim()) {
        match instruction.chars().next() {
            Some('L') => position.turn_left(),
            Some('R') => position.turn_right(),
            _ => return Err(anyhow!("invalid instruction: '{}'", instruction)),
        }
        let blocks = match instruction.get(1..).map(|str| str.parse()) {
            Some(Ok(x)) => x,
            _ => return Err(anyhow!("invalid instruction: '{}'", instruction)),
        };
        for _ in 0..blocks {
            position.step();
            if !visited.insert(position.coordinates) {
                return Ok(position.distance());
            }
        }
    }
    Err(anyhow!("Easter Bunny HQ not found"))
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug)]
struct Position {
    direction: Direction,
    coordinates: (i32, i32),
}

impl Position {
    fn turn_right(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::East => self.direction = Direction::South,
            Direction::South => self.direction = Direction::West,
            Direction::West => self.direction = Direction::North,
        }
    }

    fn turn_left(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::West,
            Direction::West => self.direction = Direction::South,
            Direction::South => self.direction = Direction::East,
            Direction::East => self.direction = Direction::North,
        }
    }

    fn walk(&mut self, blocks: i32) {
        match self.direction {
            Direction::North => self.coordinates.1 += blocks,
            Direction::West => self.coordinates.0 += blocks,
            Direction::South => self.coordinates.1 -= blocks,
            Direction::East => self.coordinates.0 -= blocks,
        }
    }

    fn step(&mut self) {
        match self.direction {
            Direction::North => self.coordinates.1 += 1,
            Direction::West => self.coordinates.0 += 1,
            Direction::South => self.coordinates.1 -= 1,
            Direction::East => self.coordinates.0 -= 1,
        }
    }

    fn distance(&self) -> i32 {
        self.coordinates.0.abs() + self.coordinates.1.abs()
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            direction: Direction::North,
            coordinates: Default::default(),
        }
    }
}

#[test]
fn part_one_example1() -> Result<()> {
    assert_eq!(part_one("R2, L3")?, 5);
    Ok(())
}

#[test]
fn part_one_example2() -> Result<()> {
    assert_eq!(part_one("R2, R2, R2")?, 2);
    Ok(())
}

#[test]
fn part_one_example3() -> Result<()> {
    assert_eq!(part_one("R5, L5, R5, R3")?, 12);
    Ok(())
}

#[test]
fn part_two_example1() -> Result<()> {
    assert_eq!(part_two("R8, R4, R4, R8")?, 4);
    Ok(())
}
