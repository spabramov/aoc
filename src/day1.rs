use crate::utils::read_lines;

use anyhow::anyhow;
use itertools::{izip, Itertools};
use std::collections::HashMap;

pub fn distance(file_path: &str) -> anyhow::Result<i32> {
    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = read_lines(file_path)?
        .map_while(|line| line.ok())
        .map(|line| -> anyhow::Result<(i32, i32)> {
            let (id1, id2) = line
                .split_once(' ')
                .ok_or(anyhow!("numbers must be separated by ' '"))?;

            Ok((id1.parse()?, id2.trim().parse()?))
        })
        .process_results(|iter| iter.unzip())?;

    list1.sort();
    list2.sort();

    let sum = izip!(list1, list2).fold(0, |sum, (left, right)| sum + (left - right).abs());

    Ok(sum)
}

pub fn similarity(file_path: &str) -> anyhow::Result<i32> {
    let mut map = HashMap::<i32, i32>::new();
    let list = read_lines(file_path)?
        .map_while(|line| line.ok())
        .map(|line| -> anyhow::Result<i32> {
            let (id1, id2) = line
                .split_once(' ')
                .ok_or(anyhow!("numbers must be separated by ' '"))?;

            let id1 = id1.parse()?;
            let id2 = id2.trim().parse()?;

            *map.entry(id2).or_insert(0) += 1;

            Ok(id1)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let sum = list
        .iter()
        .fold(0, |acc, val| acc + val * map.get(val).unwrap_or(&0));

    Ok(sum)
}

#[cfg(test)]
mod tests {

    #[test]
    fn distance() {
        let value = super::distance("data/1.txt").unwrap();
        assert!(value == 1530215, "{value:?}")
    }

    #[test]
    fn similarity() {
        let value = super::similarity("data/1.txt").unwrap();
        assert!(value == 26800609, "{value:?}")
    }
}
