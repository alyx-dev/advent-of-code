use anyhow::{Context, Result};

pub const YEAR: u32 = 2019;
pub const DAY: u32 = 1;

pub fn part_one(input: &str) -> Result<u32> {
    let mut fuel = 0;
    for mass in input.trim().lines() {
        let mass = mass.parse().with_context(|| format!("invalid mass: '{}'", mass))?;
        fuel += fuel_requirement(mass);
    }
    Ok(fuel)
}

pub fn part_two(input: &str) -> Result<i32> {
    let mut fuel = 0;
    for mass in input.trim().lines() {
        let mass = mass.parse().with_context(|| format!("invalid mass: '{}'", mass))?;
        fuel += recursive_fuel_requirement(mass);
    }
    Ok(fuel)
}

fn fuel_requirement(mass: u32) -> u32 {
    mass / 3 - 2
}

fn recursive_fuel_requirement(mass: i32) -> i32 {
    let mut mass = mass;
    let mut fuel = 0;
    loop {
        mass = mass / 3 - 2;
        if mass > 0 {
            fuel += mass
        } else {
            break;
        }
    }
    fuel
}

#[test]
fn part_one_example1() {
    assert_eq!(fuel_requirement(12), 2);
}

#[test]
fn part_one_example2() {
    assert_eq!(fuel_requirement(14), 2);
}

#[test]
fn part_one_example3() {
    assert_eq!(fuel_requirement(1969), 654);
}

#[test]
fn part_one_example4() {
    assert_eq!(fuel_requirement(100756), 33583);
}

#[test]
fn part_two_example1() {
    assert_eq!(recursive_fuel_requirement(14), 2);
}

#[test]
fn part_two_example2() {
    assert_eq!(recursive_fuel_requirement(1969), 966);
}

#[test]
fn part_rwo_example3() {
    assert_eq!(recursive_fuel_requirement(100756), 50346);
}
