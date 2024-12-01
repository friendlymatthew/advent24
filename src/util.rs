use anyhow::{anyhow, Result};

pub fn parse_str_to_usize(s: &str) -> Result<usize> {
    let n = s
        .parse::<usize>()
        .map_err(|_| anyhow!("Failed to cast {} into usize", s))?;

    Ok(n)
}
