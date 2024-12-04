use std::num::ParseIntError;

use crate::utils::read_lines;

fn parse_levels(s: &str) -> Result<Vec<i32>, ParseIntError> {
    s.split(' ')
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
}

pub fn is_safe(levels: &Vec<i32>) -> bool {
    levels
        .as_slice()
        .windows(2)
        .fold((0i8, true), |(dir, safe), parts| {
            if safe & (parts[0] < parts[1]) & (parts[1] - parts[0] <= 3) {
                (1, (dir >= 0))
            } else if safe & (parts[0] > parts[1]) & (parts[0] - parts[1] <= 3) {
                (-1, (dir <= 0))
            } else {
                (0, false)
            }
        })
        .1
}

pub fn is_safe_with_dampener(levels: &Vec<i32>) -> bool {
    if is_safe(levels) {
        return true;
    }

    (0..levels.len())
        .map(|i| {
            let mut variation: Vec<i32> = Vec::new();
            variation.extend_from_slice(&levels[0..i]);
            variation.extend_from_slice(&levels[i + 1..]);
            variation
        })
        .any(|l| is_safe(&l))
}

pub fn safe_levels(file_path: &str) -> Result<usize, anyhow::Error> {
    Ok(read_lines(file_path)?
        .map_while(|line| line.ok())
        .map(|x| parse_levels(&x))
        .map(|x| x.map(|i| is_safe(&i)))
        .collect::<Result<Vec<bool>, _>>()?
        .into_iter()
        .filter(|x| *x)
        .count())
}

pub fn safe_levels_with_dampener(file_path: &str) -> Result<usize, anyhow::Error> {
    Ok(read_lines(file_path)?
        .map_while(|line| line.ok())
        .map(|x| parse_levels(&x))
        .map(|x| x.map(|i| is_safe_with_dampener(&i)))
        .collect::<Result<Vec<bool>, _>>()?
        .into_iter()
        .filter(|x| *x)
        .count())
}
mod tests {

    #[test]
    fn one() {
        let value = super::safe_levels("data/2.txt").unwrap();
        assert!(value == 299, "{value:?}")
    }

    #[test]
    fn two() {
        let value = super::safe_levels_with_dampener("data/2.txt").unwrap();
        assert!(value == 364, "{value:?}")
    }
}
