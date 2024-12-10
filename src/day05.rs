use std::{cmp::Ordering, collections::HashSet};

use crate::utils::read_lines;
type Rules = HashSet<(usize, usize)>;
type Queues = Vec<Vec<usize>>;

fn extract_queues(file_name: &str) -> anyhow::Result<(Rules, Queues)> {
    let mut map = HashSet::new();
    let mut queues = vec![];
    for line in read_lines(file_name)?.map_while(|line| line.ok()) {
        if line.is_empty() {
            continue;
        }

        if let Some((left, right)) = line.split_once('|') {
            let left: usize = left.parse()?;
            let right: usize = right.parse()?;
            map.insert((left, right));
        } else {
            queues.push(
                line.split(',')
                    .map(|v| v.parse::<usize>())
                    .collect::<Result<Vec<usize>, _>>()?,
            );
        }
    }

    Ok((map, queues))
}

fn is_in_order(queue: &[usize], map: &Rules) -> bool {
    (0..queue.len()).all(|i| (i + 1..queue.len()).all(|j| !map.contains(&(queue[j], queue[i]))))
}

pub fn middle_numbers(file_name: &str) -> anyhow::Result<usize> {
    let (map, queues) = extract_queues(file_name)?;
    Ok(queues
        .iter()
        .filter(|queue| is_in_order(queue, &map))
        .map(|queue| queue[queue.len() / 2])
        .sum())
}

pub fn middle_numbers_of_unordered(file_name: &str) -> anyhow::Result<usize> {
    let (map, queues) = extract_queues(file_name)?;
    Ok(queues
        .into_iter()
        .filter(|queue| !is_in_order(queue, &map))
        .map(|mut queue| {
            queue.sort_by(|a, b| {
                if map.contains(&(*a, *b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            queue
        })
        .map(|queue| queue[queue.len() / 2])
        .sum())
}

#[cfg(test)]
mod test {

    #[test]
    fn middle_numbers() {
        let value = super::middle_numbers("data/05.in").unwrap();

        assert_eq!(value, 4462)
    }
    #[test]
    fn middle_numbers_unordered() {
        let value = super::middle_numbers_of_unordered("data/05.in").unwrap();

        assert_eq!(value, 6767)
    }
}
