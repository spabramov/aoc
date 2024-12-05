use std::usize;

use crate::utils::read_lines;

fn search_xmas(r: usize, c: usize, r_mode: i8, c_mode: i8, input: &[Vec<char>]) -> Option<u8> {
    const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

    for (i, letter) in WORD.iter().enumerate() {
        let x: usize = r.checked_add_signed(r_mode as isize * i as isize)?;
        let y: usize = c.checked_add_signed(c_mode as isize * i as isize)?;

        if x >= input.len() || y >= input[x].len() {
            return None;
        }

        if input[x][y] != *letter {
            return None;
        }
    }

    Some(1)
}

fn search_x_mas(r: usize, c: usize, input: &[Vec<char>]) -> Option<u8> {
    if !(1..input.len() - 1).contains(&r) || !(1..input[r].len() - 1).contains(&c) {
        return None;
    };

    if input[r][c] != 'A' {
        return None;
    }

    let strike1 = (input[r - 1][c - 1], input[r + 1][c + 1]);
    let strike2 = (input[r - 1][c + 1], input[r + 1][c - 1]);

    match (strike1, strike2) {
        (('M', 'S'), ('M', 'S'))
        | (('M', 'S'), ('S', 'M'))
        | (('S', 'M'), ('M', 'S'))
        | (('S', 'M'), ('S', 'M')) => Some(1),
        _ => None,
    }
}

pub fn ceres_search(file_path: &str) -> anyhow::Result<u32> {
    let input: Vec<Vec<char>> = read_lines(file_path)?
        .map_while(|x| Some(x.ok()?.chars().collect()))
        .collect();

    let mut count: u32 = 0;
    for row in input.iter().enumerate() {
        for column in row.1.iter().enumerate() {
            if *column.1 == 'X' {
                for r_mode in -1..=1 {
                    for c_mode in -1..=1 {
                        if let Some(cnt) = search_xmas(row.0, column.0, r_mode, c_mode, &input) {
                            count += cnt as u32;
                        }
                    }
                }
            }
        }
    }

    Ok(count)
}

pub fn ceres_search_x_mas(file_path: &str) -> anyhow::Result<u32> {
    let input: Vec<Vec<char>> = read_lines(file_path)?
        .map_while(|x| Some(x.ok()?.chars().collect()))
        .collect();

    let mut count: u32 = 0;
    for row in input.iter().enumerate() {
        for column in row.1.iter().enumerate() {
            if let Some(cnt) = search_x_mas(row.0, column.0, &input) {
                count += cnt as u32;
            }
        }
    }

    Ok(count)
}
#[cfg(test)]
mod test {

    #[test]
    fn ceres_search() {
        let value = super::ceres_search("data/4.txt").unwrap();
        assert_eq!(value, 2458)
    }
    #[test]
    fn ceres_search_x_mas() {
        let value = super::ceres_search_x_mas("data/4.txt").unwrap();
        assert_eq!(value, 1945)
    }
}
