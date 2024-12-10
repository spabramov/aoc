use std::collections::HashSet;

use crate::utils::{read_lines, try_move, Direction, Location};

type TrailMap = Vec<Vec<usize>>;
pub fn read_map(filename: &str) -> anyhow::Result<TrailMap> {
    Ok(read_lines(filename)?
        .map_while(|line| line.ok())
        .map(|line| {
            line.chars()
                .map_while(|c| c.to_digit(10).map(|c| c as usize))
                .collect()
        })
        .collect())
}

fn try_step(loc: &Location, map: &TrailMap, dir: Direction) -> Option<Location> {
    let new = try_move(loc, dir)?;

    if new.in_bounds(map) && map[loc.0][loc.1] + 1 == map[new.0][new.1] {
        Some(new)
    } else {
        None
    }
}

pub fn add_trail_peaks(start: Location, map: &TrailMap, set: &mut HashSet<Location>) {
    if map[start.0][start.1] == 9 {
        set.insert(start);
    } else {
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .for_each(|dir| {
                if let Some(loc) = try_step(&start, map, dir) {
                    add_trail_peaks(loc, map, set);
                };
            });
    }
}

pub fn count_trails(start: Location, map: &TrailMap) -> usize {
    if map[start.0][start.1] == 9 {
        1
    } else {
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .map(|dir| {
                if let Some(loc) = try_step(&start, map, dir) {
                    count_trails(loc, map)
                } else {
                    0
                }
            })
            .sum()
    }
}
#[cfg(test)]
mod test {
    use crate::day10::count_trails;

    use super::Location;
    use super::{add_trail_peaks, read_map, try_step};
    use std::collections::HashSet;

    #[test]
    fn test_step() {
        let loc = Location(0, 0);
        let map = vec![vec![1, 2], vec![3, 4]];

        assert!(matches!(try_step(&loc, &map, (1, 0)), None));
        assert!(matches!(try_step(&loc, &map, (0, 1)), Some(Location(0, 1))));
        assert!(matches!(try_step(&loc, &map, (-1, 0)), None));
        assert!(matches!(try_step(&loc, &map, (0, -1)), None));
    }

    #[test]
    fn hoof_it() {
        let input = read_map("data/10.in").expect("Couldn't read file");
        let value = (&input)
            .iter()
            .enumerate()
            .map(|(row, vec)| {
                vec.iter()
                    .enumerate()
                    .map(|(col, &val)| {
                        if val == 0 {
                            let mut peaks: HashSet<Location> = HashSet::new();
                            add_trail_peaks(Location(row, col), &input, &mut peaks);
                            peaks.len()
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();

        assert_eq!(value, 538);
    }

    #[test]
    fn hoof_it_rating() {
        let input = read_map("data/10.in").expect("Couldn't read file");
        let value = (&input)
            .iter()
            .enumerate()
            .map(|(row, vec)| {
                vec.iter()
                    .enumerate()
                    .map(|(col, &val)| {
                        (val == 0)
                            .then_some(count_trails(Location(row, col), &input))
                            .unwrap_or(0)
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();

        assert_eq!(value, 1110);
    }
}
