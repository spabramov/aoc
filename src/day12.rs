use std::collections::HashSet;

use crate::utils::{read_lines, try_move, Location};

type GardenMap = Vec<Vec<char>>;
pub fn read_map(filename: &str) -> anyhow::Result<GardenMap> {
    Ok(read_lines(filename)?
        .map_while(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect())
}

pub fn get_region(
    loc: Location,
    map: &GardenMap,
    visited: &mut HashSet<Location>,
) -> (usize, usize, isize) {
    let name = map[loc.0][loc.1];
    let mut area: usize = 1;
    let mut perimiter: usize = 4;
    let mut corners: isize = 0;
    if !visited.insert(loc.clone()) {
        panic!("Location {loc:?} visited twice");
    }

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .filter(|&dir| {
            if let Some(new) = try_move(&loc, dir) {
                if new.in_bounds(map) && map[new.0][new.1] == name {
                    if visited.contains(&new) {
                        perimiter = perimiter.checked_sub(1).expect("perimiter going sub-zero!");
                        true
                    } else {
                        let (new_area, new_perim, new_corners) = get_region(new, map, visited);
                        area += new_area;
                        perimiter = perimiter.checked_sub(1).expect("perimiter going sub-zero!");
                        perimiter += new_perim;
                        corners += new_corners;
                        true
                    }
                } else {
                    false
                }
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    corners += if dirs.is_empty() || dirs.len() == 4 {
        4
    } else {
        let sum = dirs
            .iter()
            .fold((0, 0), |(ax, ay), (x, y)| (ax + x, ay + y));

        if sum.0 != 0 || sum.1 != 0 {
            2
        } else {
            0
        }
    };

    // off-by-one 2x2 square. In this case we have to subtract 4 corners
    if Location(loc.0 + 1, loc.1 + 1).in_bounds(map)
        && map[loc.0 + 1][loc.1] == name
        && map[loc.0][loc.1 + 1] == name
        && map[loc.0 + 1][loc.1 + 1] == name
    {
        corners -= 4;
    }
    // println!("{loc:?}: area={area}, perimiter={perimiter}");
    (area, perimiter, corners)
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::utils::Location;

    use super::{get_region, read_map};

    #[test]
    fn garden_groups() {
        let map = read_map("data/12.in").unwrap();
        let mut visited: HashSet<Location> = HashSet::new();

        let (by_perim, by_sides) = map
            .iter()
            .enumerate()
            .map(|(row, vec)| {
                vec.iter()
                    .enumerate()
                    .map(|(col, _)| {
                        let loc = Location(row, col);
                        if !visited.contains(&loc) {
                            let mut region = HashSet::new();
                            let (area, perimeter, sides) = get_region(loc, &map, &mut region);
                            visited.extend(region);
                            (area * perimeter, area * (sides as usize))
                        } else {
                            (0, 0)
                        }
                    })
                    .fold((0, 0), |(accp, accs), (p, s)| (accp + p, accs + s))
            })
            .fold((0, 0), |(accp, accs), (p, s)| (accp + p, accs + s));

        assert_eq!(by_perim, 1533644, "by perimeter");
        assert_eq!(by_sides, 936718, "by sides");
    }
}
