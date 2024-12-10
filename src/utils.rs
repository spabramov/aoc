use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub type Location = (usize, usize);
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

pub fn try_move(loc: Location, dir: Direction) -> Option<Location> {
    Some((
        loc.0.checked_add_signed(dir.0)?,
        loc.1.checked_add_signed(dir.1)?,
    ))
}
