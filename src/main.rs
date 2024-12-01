use anyhow::Result;

use aoc::day1::total_distance;

fn main() -> Result<()> {
    let data = std::fs::read_to_string("./tests/day1-1.txt")?;
    total_distance(data)?;
    Ok(())
}
