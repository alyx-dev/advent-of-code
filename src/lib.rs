use std::io::{stdin, Read};

use anyhow::{Context, Result};

/// Read input from standard input.
pub fn input_from_stdin() -> Result<String> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).context("stdin error")?;
    Ok(input)
}
