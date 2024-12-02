use crate::utils::read_lines;

use itertools::izip;
use std::collections::HashMap;

pub fn distance(file_path: &str) -> Result<i32, anyhow::Error> {
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for line in read_lines(file_path)?.flatten() {
        let (id1, id2) = line
            .split_once(" ")
            .expect("numbers must be separated by ' '");
        let id1: i32 = id1.trim().parse().expect("not an iteger");
        let id2: i32 = id2.trim().parse().expect("not an iteger");

        list1.push(id1);
        list2.push(id2);
    }

    list1.sort();
    list2.sort();

    let sum = izip!(list1, list2).fold(0, |acc, (left, right)| acc + (left - right).abs());

    Ok(sum)
}

pub fn similarity(file_path: &str) -> Result<i32, anyhow::Error> {
    let mut list1: Vec<i32> = vec![];
    let mut map2: HashMap<i32, i32> = HashMap::new();

    for line in read_lines(file_path)?.flatten() {
        let (id1, id2) = line
            .split_once(" ")
            .expect("numbers must be separated by ' '");
        let id1: i32 = id1.trim().parse().expect("not an iteger");
        let id2: i32 = id2.trim().parse().expect("not an iteger");

        list1.push(id1);
        *map2.entry(id2).or_insert(0) += 1;
    }

    let sum = list1
        .iter()
        .fold(0, |acc, val| acc + val * map2.get(val).unwrap_or(&0));

    Ok(sum)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let value = distance("data/1.txt").unwrap();
        assert!(value == 1530215, "{value:?}")
    }

    #[test]
    fn two() {
        let value = similarity("data/1.txt").unwrap();
        assert!(value == 26800609, "{value:?}")
    }
}
