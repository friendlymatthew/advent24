use std::collections::BTreeMap;
use std::str::FromStr;

use anyhow::{ensure, Result};

pub fn day_1_part_1() -> Result<usize> {
    let content = std::fs::read("./tests/day1.txt")?;
    let total_distance = total_distance(&content)?;

    Ok(total_distance)
}

pub fn day_1_part_2() -> Result<usize> {
    let content = std::fs::read("./tests/day1.txt")?;
    let sim_score = similarity_score(&content)?;

    Ok(sim_score)
}

// since each number takes 5 bytes of space, we can do better than usize::from_str
const NUM_SIZE: usize = 5;
fn parse_number_scalar(num_buffer: &[u8]) -> Result<usize> {
    ensure!(
        num_buffer.len() == NUM_SIZE,
        "Expected number to take length of 5"
    );

    let num = (num_buffer[0] - b'0') as usize * 10000
        + (num_buffer[1] - b'0') as usize * 1000
        + (num_buffer[2] - b'0') as usize * 100
        + (num_buffer[3] - b'0') as usize * 10
        + (num_buffer[4] - b'0') as usize;

    Ok(num)
}

// Whitespace is always 3 bytes.
const WHITESPACE_SIZE: usize = 3;

// \n is 1 byte.
const NEWLINE_SIZE: usize = 1;

// take advantage of the fact that the input data is the following:
// a: 5 bytes, whitespace: 3 bytes, b: 5 bytes, \n: 1 byte
const LINE_SIZE: usize = NUM_SIZE + WHITESPACE_SIZE + NUM_SIZE + NEWLINE_SIZE;

fn total_distance(content: &[u8]) -> Result<usize> {
    let num_lines = (content.len() + NEWLINE_SIZE) / LINE_SIZE;

    let mut left = Vec::with_capacity(num_lines);
    let mut right = Vec::with_capacity(num_lines);

    for i in 0..num_lines {
        let mut start = i * LINE_SIZE;

        let left_num = &content[start..start + NUM_SIZE];
        start += NUM_SIZE + 3;
        let right_num = &content[start..start + NUM_SIZE];

        left.push(parse_number_scalar(left_num)?);
        right.push(parse_number_scalar(right_num)?);
    }

    left.sort();
    right.sort();

    let total_distance = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    Ok(total_distance)
}

fn similarity_score(content: &[u8]) -> Result<usize> {
    let num_lines = (content.len() + NEWLINE_SIZE) / LINE_SIZE;

    let mut left = Vec::with_capacity(num_lines);
    let mut right = BTreeMap::new();

    for i in 0..num_lines {
        let mut start = i * LINE_SIZE;

        let left_num = &content[start..start + NUM_SIZE];
        start += NUM_SIZE + 3;
        let right_num = &content[start..start + NUM_SIZE];

        left.push(parse_number_scalar(left_num)?);
        right
            .entry(parse_number_scalar(right_num)?)
            .and_modify(|n| *n += 1)
            .or_insert(1_usize);
    }

    let sim_score = left
        .into_iter()
        .map(|l| {
            if right.contains_key(&l) {
                let count = *right.get(&l).unwrap();
                l * count
            } else {
                0
            }
        })
        .sum();

    Ok(sim_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() -> Result<()> {
        assert_eq!(day_1_part_1()?, 1388114);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        assert_eq!(day_1_part_2()?, 23529853);
        Ok(())
    }
}
