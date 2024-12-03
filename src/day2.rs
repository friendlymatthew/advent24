use std::str::FromStr;

use anyhow::{anyhow, Result};

fn safe_reports(content: &str) -> Result<usize> {
    let mut num_safe = 0;

    for line in content.lines() {
        let mut split = line.split_whitespace();

        // we'll do two passes. One for increasing, one for decreasing

        let mut is_increasing = true;

        let mut prev = usize::MAX;
        while let Some(num_str) = split.next() {
            let curr = usize::from_str(num_str)?;

            if prev != usize::MAX {
                if prev < curr {
                    if curr - prev > 3 {
                        is_increasing = false;
                        break;
                    }
                } else {
                    is_increasing = false;
                    break;
                }
            }

            prev = curr;
        }

        if is_increasing {
            num_safe += 1;
            continue;
        }

        let mut split = line.split_whitespace();
        let mut is_decreasing = true;

        let mut prev = usize::MAX;
        while let Some(num_str) = split.next() {
            let curr = usize::from_str(num_str)?;

            if prev != usize::MAX {
                if curr < prev {
                    if prev - curr > 3 {
                        is_decreasing = false;
                        break;
                    }
                } else {
                    is_decreasing = false;
                    break;
                }
            }

            prev = curr;
        }

        if is_decreasing {
            num_safe += 1;
        }
    }

    Ok(num_safe)
}

fn valid_record(levels: &[usize]) -> bool {
    let mut prev = usize::MAX;

    let mut is_decreasing = true;

    for &curr in levels {
        if prev != usize::MAX {
            if curr < prev {
                if prev - curr > 3 {
                    is_decreasing = false;
                    break;
                }
            } else {
                is_decreasing = false;
                break;
            }
        }

        prev = curr;
    }

    if is_decreasing {
        return true;
    }

    prev = usize::MAX;
    let mut is_increasing = true;

    for &curr in levels {
        if prev != usize::MAX {
            if prev < curr {
                if curr - prev > 3 {
                    is_increasing = false;
                    break;
                }
            } else {
                is_increasing = false;
                break;
            }
        }

        prev = curr;
    }

    if is_increasing {
        return true;
    }

    false
}

fn safe_reports_brute_force(content: &str) -> Result<usize> {
    let mut num_safe = 0;

    for line in content.lines() {
        let mut split = line.split_whitespace();
        let mut nums = Vec::new();

        while let Some(num_str) = split.next() {
            nums.push(usize::from_str(num_str)?);
        }

        for i in 0..nums.len() {
            let levels = nums
                .iter()
                .enumerate()
                .filter_map(|(j, &num)| if i != j { Some(num) } else { None })
                .collect::<Vec<_>>();

            if valid_record(&levels) {
                num_safe += 1;
                break;
            }
        }
    }

    Ok(num_safe)
}

fn _safe_reports_tolerate_bad_level(content: &str) -> Result<usize> {
    let mut num_safe = 0;

    for line in content.lines() {
        let mut split = line.split_whitespace();

        // we'll do two passes. One for increasing, one for decreasing

        let mut is_bad = false;
        let mut is_increasing = true;

        let mut prev = usize::from_str(split.next().ok_or(anyhow!("Improper"))?)?;

        while let Some(num_str) = split.next() {
            let curr = usize::from_str(num_str)?;

            if prev < curr {
                if curr - prev > 3 {
                    if is_bad {
                        is_increasing = false;
                        break;
                    }

                    is_bad = true;
                    continue;
                }

                prev = curr;
            } else {
                if is_bad {
                    is_increasing = false;
                    break;
                }

                is_bad = true;
                continue;
            }
        }

        if is_increasing {
            num_safe += 1;
            dbg!(line);
            continue;
        } else {
            if safe_reports(&line[1..])? == 1 {
                num_safe += 1;
                dbg!(line);
                continue;
            }
        }

        is_bad = false;

        let mut split = line.split_whitespace();
        let mut is_decreasing = true;

        prev = usize::from_str(split.next().ok_or(anyhow!("Improper"))?)?;

        while let Some(num_str) = split.next() {
            let curr = usize::from_str(num_str)?;

            if curr < prev {
                if prev - curr > 3 {
                    if is_bad {
                        is_decreasing = false;
                        break;
                    }

                    is_bad = true;
                    continue;
                }

                prev = curr;
            } else {
                if is_bad {
                    is_decreasing = false;
                    break;
                }

                is_bad = true;
            }
        }

        if is_decreasing {
            num_safe += 1;
            dbg!(line);
        } else {
            if safe_reports(&line[1..])? == 1 {
                num_safe += 1;
                dbg!(line);
            }
        }
    }

    Ok(num_safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_part_1() -> Result<()> {
        let content = std::fs::read_to_string("./tests/day2_test.txt")?;
        assert_eq!(2, safe_reports(&content)?);

        Ok(())
    }

    #[test]
    pub fn part_1() -> Result<()> {
        let content = std::fs::read_to_string("./tests/day2.txt")?;
        assert_eq!(321, safe_reports(&content)?);

        Ok(())
    }

    #[test]
    pub fn test_part_2() -> Result<()> {
        let content = std::fs::read_to_string("./tests/day2_test.txt")?;
        assert_eq!(7, safe_reports_brute_force(&content)?);

        Ok(())
    }

    #[test]
    pub fn part_2() -> Result<()> {
        let content = std::fs::read_to_string("./tests/day2.txt")?;
        assert_eq!(386, safe_reports_brute_force(&content)?);

        Ok(())
    }
}
