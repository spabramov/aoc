use std::error::Error;
use std::str::FromStr;

use anyhow::anyhow;

use crate::utils::{read_lines, Direction, Location};

pub fn extract<T: FromStr>(line: &str, sep: char) -> anyhow::Result<(T, T)>
where
    T: FromStr,
    T::Err: Send + Sync + Error + 'static,
{
    let (_, right) = line.split_once(':').ok_or(anyhow!("split ':' failed"))?;
    let (x, y) = right.split_once(',').ok_or(anyhow!("split ',' failed"))?;

    Ok((
        x.split_once(sep)
            .ok_or(anyhow!("1st split {sep} failed"))?
            .1
            .trim()
            .parse::<T>()?,
        y.split_once(sep)
            .ok_or(anyhow!("2nd split {sep} failed"))?
            .1
            .trim()
            .parse::<T>()?,
    ))
}
pub fn read_machines(filename: &str) -> anyhow::Result<Vec<(Direction, Direction, Location)>> {
    read_lines(filename)?
        .map_while(|line| line.ok())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|lines| -> anyhow::Result<_> {
            Ok((
                extract::<isize>(&lines[0], '+')?,
                extract::<isize>(&lines[1], '+')?,
                extract::<usize>(&lines[2], '=')?.into(),
            ))
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

pub fn solve_kramer(target: &Location, a: Direction, b: Direction, corr: isize) -> Option<isize> {
    let t = (target.0 as isize + corr, target.1 as isize + corr);

    let det = a.0 * b.1 - b.0 * a.1;
    let xdet = t.0 * b.1 - b.0 * t.1;
    let ydet = a.0 * t.1 - t.0 * a.1;

    if xdet % det == 0 && ydet % det == 0 {
        Some(xdet / det * 3 + ydet / det)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use std::ops::Add;

    use crate::{day13::solve_kramer, utils::Location};

    use super::read_machines;

    #[test]
    fn loc_cmp() {
        assert!(Location(0, 0) < Location(1, 0));
        assert!(Location(1, 1) > Location(0, 0));
        assert_eq!(Location(1, 1), Location(1, 1));

        assert_eq!(Location(0, 1).add((2, 3)), Some(Location(2, 4)));
    }

    #[test]
    fn claw_contraption() {
        let values: isize = read_machines("data/13.in")
            .unwrap()
            .into_iter()
            .filter_map(|(a, b, target)| solve_kramer(&target, a, b, 0))
            .sum();

        assert_eq!(values, 29436);
    }

    #[test]
    fn claw_contraption_corr() {
        let values: isize = read_machines("data/13.in")
            .unwrap()
            .into_iter()
            .filter_map(|(a, b, target)| solve_kramer(&target, a, b, 10_000_000_000_000))
            .sum();

        assert_eq!(values, 103729094227877);
    }
}
