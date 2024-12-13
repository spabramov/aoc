use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;
use std::path::Path;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Location(pub usize, pub usize);
pub type Direction = (isize, isize);

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn try_move(loc: &Location, dir: Direction) -> Option<Location> {
    Some(Location(
        loc.0.checked_add_signed(dir.0)?,
        loc.1.checked_add_signed(dir.1)?,
    ))
}

impl Location {
    pub fn in_bounds<T>(&self, map: &[Vec<T>]) -> bool {
        self.0 < map.len() && self.1 < map[self.0].len()
    }
}

impl From<(usize, usize)> for Location {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0, value.1)
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.0, self.0).partial_cmp(&(other.0, other.1))
    }
}
impl Add<Direction> for Location {
    type Output = Option<Location>;

    fn add(self, rhs: Direction) -> Self::Output {
        Some(Location(
            self.0.checked_add_signed(rhs.0)?,
            self.1.checked_add_signed(rhs.1)?,
        ))
    }
}
