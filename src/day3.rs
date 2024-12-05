use core::panic;
use regex::Regex;
use std::{fs, num::ParseIntError};

pub fn sum_of_muls(file_name: &str) -> anyhow::Result<i32> {
    let pattern = Regex::new(r"mul\((\d*),(\d*)\)").expect("pattern is not correct");

    let content = fs::read_to_string(file_name)?;

    let out = pattern
        .captures_iter(&content)
        .map(|x| x.extract())
        .map(|(_, [left, right])| -> anyhow::Result<(i32, i32)> {
            Ok((left.parse()?, right.parse()?))
        })
        .try_fold(0, |acc, result| -> anyhow::Result<i32> {
            let (left, right) = result?;
            Ok(acc + left * right)
        });

    out
}

pub fn sum_of_muls_and_donts(file_name: &str) -> Result<i32, anyhow::Error> {
    let pattern = Regex::new(r"((do)()()\(\)|(don't)()()|(mul)\((\d*),(\d*)\))")?;
    let content = fs::read_to_string(file_name)?;

    let result = pattern
        .captures_iter(&content)
        .map(|x| x.extract())
        .map(|(_, [_, op, left, right])| -> (&str, &str, &str) { (op, left, right) })
        .try_fold((0, 1), |acc, result| -> Result<(i32, i32), ParseIntError> {
            let (op, left, right) = result;
            let (sum, coef) = acc;
            match op {
                "do" => Ok((sum, 1)),
                "don't" => Ok((sum, 0)),
                "mul" => Ok((
                    sum + coef * left.parse::<i32>()? * right.parse::<i32>()?,
                    coef,
                )),
                _ => panic!("Unexpected op={op}"),
            }
        })?;

    Ok(result.0)
}

#[cfg(test)]
mod test {

    #[test]
    fn mull_it_over() {
        let value = super::sum_of_muls("data/3.txt").unwrap();

        assert_eq!(value, 191183308)
    }

    #[test]
    fn mull_it_over_and_dont() {
        let value = super::sum_of_muls_and_donts("data/3.txt").unwrap();

        assert_eq!(value, 92082041)
    }
}
