use std::{
    collections::{HashMap, HashSet},
    ops::Sub,
};

use itertools::Itertools;

use crate::utils::read_lines;

type Antennas = HashMap<char, HashSet<(usize, usize)>>;

fn antennas(filename: &str) -> anyhow::Result<(Antennas, (usize, usize))> {
    let mut map = HashMap::new();
    let mut dim = (0, 0);

    for (row, line) in read_lines(filename)?.map_while(|l| l.ok()).enumerate() {
        dim.0 += 1;
        dim.1 = 0;
        for (col, ch) in line.chars().enumerate() {
            dim.1 += 1;
            if ch != '.' {
                map.entry(ch).or_insert(HashSet::new()).insert((row, col));
            };
        }
    }

    Ok((map, dim))
}

fn harmonic(
    a: (usize, usize),
    b: (usize, usize),
    n: isize,
    dim: (usize, usize),
) -> Option<(usize, usize)> {
    let loc = (
        b.0.checked_add_signed(n * (b.0 as isize - a.0 as isize))?,
        b.1.checked_add_signed(n * (b.1 as isize - a.1 as isize))?,
    );

    if loc.0 < dim.0 && loc.1 < dim.1 {
        Some(loc)
    } else {
        None
    }
}

pub fn resonant_collinearity(filename: &str) -> anyhow::Result<usize> {
    let (antennas, dim) = antennas(filename)?;
    let mut antinodes = HashSet::new();

    for (_, locations) in antennas.iter() {
        for (&a, &b) in locations.iter().permutations(2).map(|vec| (vec[0], vec[1])) {
            if let Some((x, y)) = harmonic(a, b, 1, dim) {
                antinodes.insert((x, y));
            }

            if let Some((x, y)) = harmonic(a, b, -2, dim) {
                antinodes.insert((x, y));
            }
        }
    }

    dbg!(&antinodes);
    Ok(antinodes.len())
}

pub fn resonant_collinearity_harm(filename: &str) -> anyhow::Result<usize> {
    let (antennas, dim) = antennas(filename)?;
    let mut antinodes: HashSet<_> = HashSet::new();

    for (_, locations) in antennas.iter() {
        for (&a, &b) in locations.iter().permutations(2).map(|vec| (vec[0], vec[1])) {
            let mut n = 0;
            while let Some((x, y)) = harmonic(a, b, n, dim) {
                antinodes.insert((x, y));
                n += 1;
            }

            n = -1;
            while let Some((x, y)) = harmonic(a, b, n, dim) {
                antinodes.insert((x, y));
                n -= 1;
            }
        }
    }

    dbg!(&antinodes);

    Ok(antinodes.len())
}
#[cfg(test)]
mod test {

    #[test]
    fn resonant_collinearity() {
        let value = super::resonant_collinearity("data/8.in").unwrap();

        assert_eq!(value, 364);
    }

    #[test]
    fn resonant_collinearity_harm() {
        let value = super::resonant_collinearity_harm("data/8.in").unwrap();

        assert_eq!(value, 0);
    }
}
