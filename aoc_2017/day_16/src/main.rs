use anyhow::Result;

use aoc_2017_day_16::*;

fn main() -> Result<()> {
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    let input = aoc::input_from_stdin()?;
    let answer = part_one(&input)?;
    println!("--> part one:");
    println!("{}", answer);
    let answer = part_two(&input)?;
    println!("--> part two:");
    println!("{}", answer);
    Ok(())
}
