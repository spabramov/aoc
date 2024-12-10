use anyhow::anyhow;

use crate::utils::read_lines;

type Equation = (usize, Vec<usize>);
fn equations(filename: &str) -> anyhow::Result<Vec<Equation>> {
    read_lines(filename)?
        .map_while(|line| line.ok())
        .map(|line| {
            let (result, line) = line.split_once(':').ok_or(anyhow!("missing ':'"))?;
            Ok((
                result.trim().parse()?,
                line.trim()
                    .split(' ')
                    .map(|val| val.trim().parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        })
        .collect()
}

fn exists_calibration(target: usize, values: &[usize], acc: usize) -> bool {
    if values.is_empty() {
        acc == target
    } else if acc > target {
        false
    } else {
        exists_calibration(target, &values[1..], acc + values[0])
            || exists_calibration(target, &values[1..], acc * values[0])
    }
}

fn exists_calibration_concat(target: usize, values: &[usize], acc: usize) -> bool {
    if values.is_empty() {
        acc == target
    } else if acc > target {
        false
    } else {
        exists_calibration_concat(target, &values[1..], acc + values[0])
            || exists_calibration_concat(target, &values[1..], acc * values[0])
            || exists_calibration_concat(
                target,
                &values[1..],
                acc * 10usize.pow(values[0].ilog10() + 1) + values[0],
            )
    }
}

pub fn bridge_repair(filename: &str) -> anyhow::Result<usize> {
    Ok(equations(filename)?
        .into_iter()
        .filter_map(|(result, values)| exists_calibration(result, &values, 0).then_some(result))
        .sum())
}

pub fn bridge_repair_concat_rec(filename: &str) -> anyhow::Result<usize> {
    Ok(equations(filename)?
        .into_iter()
        .filter_map(|(target, values)| {
            exists_calibration_concat(target, &values, 0).then_some(target)
        })
        .sum())
}

#[cfg(test)]
mod test {

    #[test]
    fn bridge_repair() {
        let value = super::bridge_repair("data/07.in").unwrap();
        assert_eq!(value, 1038838357795);
    }

    #[test]
    fn bridge_repair_concat() {
        let value = super::bridge_repair_concat_rec("data/07.in").unwrap();
        assert_eq!(value, 254136560217241);
    }
}
