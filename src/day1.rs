use std::collections::BTreeMap;
use std::str::FromStr;

use anyhow::{bail, Result};

pub fn total_distance(content: String) -> Result<usize> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in content.lines() {
        let mut split = line.split_whitespace();

        match (split.next(), split.next()) {
            (Some(left_str), Some(right_str)) => {
                left.push(usize::from_str(left_str)?);
                right.push(usize::from_str(right_str)?);
            }
            _ => bail!("Improper format."),
        }
    }

    // sort
    left.sort();
    right.sort();

    let total_distance = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    Ok(total_distance)
}

pub fn similarity_score(content: String) -> Result<usize> {
    let mut left = Vec::new();
    let mut right = BTreeMap::new();

    for line in content.lines() {
        let mut split = line.split_whitespace();

        match (split.next(), split.next()) {
            (Some(left_str), Some(right_str)) => {
                left.push(usize::from_str(left_str)?);

                let r_key = usize::from_str(right_str)?;
                right
                    .entry(r_key)
                    .and_modify(|curr| *curr += 1)
                    .or_insert(1_usize);
            }
            _ => bail!("Improper format."),
        }
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
    fn part_1_sample() -> Result<()> {
        let data = std::fs::read("./tests/day1_test.txt")?;
        let content = String::from_utf8(data)?;

        assert_eq!(11, total_distance(content)?);

        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        let data = std::fs::read("./tests/day1-1.txt")?;
        let content = String::from_utf8(data)?;
        assert_eq!(total_distance(content)?, 1388114);
        Ok(())
    }

    #[test]
    fn part_2_sample() -> Result<()> {
        let data = std::fs::read("./tests/day1_test.txt")?;
        let content = String::from_utf8(data)?;

        assert_eq!(31, similarity_score(content)?);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let data = std::fs::read("./tests/day1-1.txt")?;
        let content = String::from_utf8(data)?;
        assert_eq!(similarity_score(content)?, 23529853);
        Ok(())
    }
}
