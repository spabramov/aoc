use std::collections::{HashMap, HashSet};

use anyhow::anyhow;

use crate::utils::read_lines;

pub fn start_position(maze: &[Vec<char>]) -> anyhow::Result<(usize, usize)> {
    maze.iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
                .enumerate()
                .find_map(|(col, val)| (*val == '^').then_some((row, col)))
        })
        .ok_or(anyhow!("no initial location"))
}

pub fn rotate((x, y): (isize, isize)) -> (isize, isize) {
    (y, -x)
}

pub fn step(
    maze: &[Vec<char>],
    loc: (usize, usize),
    dir: (isize, isize),
) -> Option<((usize, usize), (isize, isize))> {
    let new_loc = (
        loc.0.checked_add_signed(dir.0)?,
        loc.1.checked_add_signed(dir.1)?,
    );

    if new_loc.0 >= maze.len() || new_loc.1 >= maze[new_loc.0].len() {
        None
    } else if maze[new_loc.0][new_loc.1] == '#' {
        Some((loc, rotate(dir)))
    } else {
        Some((new_loc, dir))
    }
}

pub fn guard_gallivant(filepath: &str) -> anyhow::Result<HashSet<(usize, usize)>> {
    let maze: Vec<Vec<_>> = read_lines(filepath)?
        .map_while(|v| v.ok())
        .map(|line| line.chars().collect())
        .collect();
    let mut seen = HashSet::new();
    let mut loc = start_position(&maze)?;
    let mut dir: (isize, isize) = (-1, 0);

    seen.insert(loc);
    while let Some((new_loc, new_dir)) = step(&maze, loc, dir) {
        loc = new_loc;
        dir = new_dir;
        seen.insert(loc);
    }

    Ok(seen)
}

fn try_loop(maze: &[Vec<char>], loc: (usize, usize), dir: (isize, isize)) -> bool {
    let (mut loc, mut dir) = (loc, dir);

    let mut seen = HashSet::new();
    let mut cnt = 0;

    // println!("{loc:?} {dir:?}");
    seen.insert((loc, dir));
    while let Some((new_loc, new_dir)) = step(maze, loc, dir) {
        if dir != new_dir && seen.contains(&(new_loc, new_dir)) {
            return true;
        }

        loc = new_loc;
        dir = new_dir;
        seen.insert((loc, dir));

        cnt += 1;
        if cnt > 100000 {
            panic!("bad");
        };
    }
    false
}

pub fn guard_gallivant_loops(filepath: &str) -> anyhow::Result<usize> {
    let mut maze: Vec<Vec<_>> = read_lines(filepath)?
        .map_while(|v| v.ok())
        .map(|line| line.chars().collect())
        .collect();
    let mut seen = HashSet::new();
    let mut obstructions = HashSet::new();
    let mut loc = start_position(&maze)?;
    let mut dir: (isize, isize) = (-1, 0);

    while let Some((new_loc, new_dir)) = step(&maze, loc, dir) {
        if new_loc != loc && !seen.contains(&new_loc) {
            let (x, y) = new_loc;
            maze[x][y] = '#';
            if try_loop(&maze, loc, rotate(dir)) {
                dbg!((x, y));
                obstructions.insert((x, y));
            };
            maze[x][y] = '.';
        }
        loc = new_loc;
        dir = new_dir;
        seen.insert(loc);
    }

    Ok(obstructions.len())
}
#[cfg(test)]
mod test {

    #[test]
    fn find_position() {
        let input = vec![vec!['.', '.', '.'], vec!['^', '.', '.']];

        let loc = super::start_position(&input).unwrap();
        assert_eq!(loc, (1, 0))
    }

    #[test]
    fn rotate() {
        assert_eq!(super::rotate((-1, 0)), (0, 1));
        assert_eq!(super::rotate((0, 1)), (1, 0));
        assert_eq!(super::rotate((1, 0)), (0, -1));
        assert_eq!(super::rotate((0, -1)), (-1, 0));
    }

    #[test]
    fn guard_gallivant() {
        let value = super::guard_gallivant("data/6.txt").unwrap().len();

        assert_eq!(value, 4988);
    }

    #[test]
    fn guard_gallivant_loop() {
        let value = super::guard_gallivant_loops("data/6.txt").unwrap();

        assert_eq!(value, 1697);
    }
}
