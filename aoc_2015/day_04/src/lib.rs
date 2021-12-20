use anyhow::Result;
use md5::{digest::generic_array::GenericArray, Digest, Md5};

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 3;

pub fn part_one(input: &str) -> Result<u32> {
    let input = input.trim();
    let mut hasher = Md5::new();
    let mut array = GenericArray::default();
    for int in 1.. {
        hasher.update(input);
        hasher.update(int.to_string());
        hasher.finalize_into_reset(&mut array);
        if array[..2] == [0, 0] && array[2] <= 0x0F {
            return Ok(int);
        }
    }
    unreachable!()
}

pub fn part_two(input: &str) -> Result<u32> {
    let input = input.trim();
    let mut hasher = Md5::new();
    let mut array = GenericArray::default();
    for int in 1.. {
        hasher.update(input);
        hasher.update(int.to_string());
        hasher.finalize_into_reset(&mut array);
        if array[..3] == [0, 0, 0] {
            return Ok(int);
        }
    }
    unreachable!()
}

#[test]
fn part_one_example1() -> Result<()> {
    assert_eq!(part_one("abcdef")?, 609043);
    Ok(())
}

#[test]
fn part_one_example2() -> Result<()> {
    assert_eq!(part_one("pqrstuv")?, 1048970);
    Ok(())
}
