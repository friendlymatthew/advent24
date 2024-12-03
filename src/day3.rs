use std::str::FromStr;

use anyhow::{ensure, Result};
use regex::Regex;

pub fn day_3_part_1() -> Result<()> {
    Ok(())
}

pub fn day_3_part_2() -> Result<()> {
    Ok(())
}

fn parse_num(n_str: &str) -> Result<usize> {
    ensure!(n_str.len() < 4 && n_str.len() > 0);
    let n = usize::from_str(n_str)?;
    Ok(n)
}

fn mul_line(mut content: &str) -> Result<usize> {
    let mut sum = 0;

    while let Some(index) = content.find("mul(") {
        content = &content[index + 4..];

        if let Some(index) = content.find(",") {
            if index > 3 {
                continue;
            }

            let left_num = match parse_num(&content[..index]) {
                Ok(n) => n,
                Err(_) => continue,
            };

            content = &content[index + 1..];

            if let Some(index) = content.find(")") {
                if index > 3 {
                    continue;
                }

                let right_num = match parse_num(&content[..index]) {
                    Ok(n) => n,
                    Err(_) => continue,
                };

                content = &content[index + 1..];

                sum += left_num * right_num;
            }
        }
    }

    Ok(sum)
}

fn mul_line_cond(mut content: &str) -> Result<usize> {
    let mut sum = 0;

    let mut enabled = true;
    let pattern = r"don't\(\)|mul\((\d{1,3}),(\d{1,3})\)|do\(\)";
    let regex = Regex::new(pattern)?;

    while let Some(mat) = regex.find(&content) {
        dbg!(mat.as_str());
        match mat.as_str() {
            "don't()" => {
                enabled = false;
            }
            "do()" => {
                enabled = true;
            }
            mul => {
                if enabled {
                    let mul = &mul["mul(".len()..];
                    let comma_index = mul.find(",").unwrap();
                    let left = usize::from_str(&mul[..comma_index])?;
                    let right = usize::from_str(&mul[comma_index + 1..mul.len() - 1])?;

                    dbg!(left, right);

                    sum += left * right;
                }
            }
        }

        content = &content[mat.end()..];
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() -> Result<()> {
        let content = std::fs::read_to_string("./tests/day3_test.txt")?;
        assert_eq!(161, mul_line(&content)?);
        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        let content = std::fs::read_to_string("./tests/day3.txt")?;
        assert_eq!(166630675, mul_line(&content)?);
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        let content = std::fs::read_to_string("./tests/day3_2_test.txt")?;
        assert_eq!(48, mul_line_cond(&content)?);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let content = std::fs::read_to_string("./tests/day3.txt")?;
        assert_eq!(93465710, mul_line_cond(&content)?);
        Ok(())
    }
}
