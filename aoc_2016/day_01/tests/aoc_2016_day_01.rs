use anyhow::Result;

use aoc_2016_day_01::*;

#[test]
fn part_one_answer() -> Result<()> {
    let input = include_str!("../input/input.txt");
    assert_eq!(part_one(input)?, 243);
    Ok(())
}

#[test]
fn part_two_answer() -> Result<()> {
    let input = include_str!("../input/input.txt");
    assert_eq!(part_two(input)?, 2);
    Ok(())
}
