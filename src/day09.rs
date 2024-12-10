use std::iter::repeat;

pub fn checksum(input: &[Option<usize>]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, c)| i * c.unwrap_or(0))
        .sum()
}
pub fn defragment(input: &mut [Option<usize>]) {
    let (mut i, mut j) = (0, input.len() - 1);
    while i < j {
        match (input[i], input[j]) {
            (_, None) => j -= 1,
            (Some(_), _) => i += 1,
            (_, Some(c)) => {
                input[i] = Some(c);
                input[j] = None;
                i += 1;
                j -= 1;
            }
        }
    }
}

pub fn defragment_whole(input: &mut [Option<usize>]) {
    let mut prev = None;

    let mut cnt = 1;
    for j in (0..input.len()).rev() {
        if input[j] == prev {
            cnt += 1;
        } else {
            if let Some(c) = prev {
                // moving file
                let mut subcnt = 0;
                let mut i = 0;
                while i <= j {
                    if input[i].is_none() {
                        subcnt += 1;
                    } else {
                        subcnt = 0;
                    }
                    if subcnt == cnt {
                        (0..cnt).for_each(|k| {
                            input[i - k] = Some(c);
                            input[j + k + 1] = None;
                        });
                        break;
                    }
                    i += 1;
                }
            }

            cnt = 1;
            prev = input[j];
        }
    }
}
pub fn read(input: &str) -> anyhow::Result<Vec<Option<usize>>> {
    let mut output: Vec<Option<usize>> = Vec::new();
    for (pos, ch) in input.chars().enumerate() {
        if let Some(len) = ch.to_digit(10) {
            if pos % 2 == 0 {
                output.extend(repeat(Some(pos / 2)).take(len as usize));
            } else {
                output.extend(repeat(None).take(len as usize));
            }
        }
    }
    Ok(output)
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use itertools::Itertools;

    fn fmt(input: &Vec<Option<usize>>) -> String {
        input
            .iter()
            .map(|u| match u {
                Some(d) => format!("{d}"),
                None => String::from("."),
            })
            .join("")
    }

    #[test]
    fn checksum() {
        let input: Vec<_> = "0099811188827773336446555566.............."
            .chars()
            .map(|c| c.to_digit(10).map(|x| x as usize))
            .collect();
        let value = super::checksum(&input);

        assert_eq!(value, 1928);
    }

    #[test]
    fn defragment() {
        let mut input: Vec<_> = "0..111....22222"
            .chars()
            .map(|c| c.to_digit(10).map(|x| x as usize))
            .collect();
        super::defragment(&mut input);
        assert_eq!(fmt(&input), "022111222......");

        input = "00...111...2...333.44.5555.6666.777.888899"
            .chars()
            .map(|c| c.to_digit(10).map(|x| x as usize))
            .collect();
        super::defragment(&mut input);
        assert_eq!(fmt(&input), "0099811188827773336446555566..............");
    }
    #[test]
    fn defragment_whole() {
        let mut input: Vec<_> = "00...111...2...333.44.5555.6666.777.888899"
            .chars()
            .map(|c| c.to_digit(10).map(|x| x as usize))
            .collect();
        super::defragment_whole(&mut input);
        assert_eq!(fmt(&input), "00992111777.44.333....5555.6666.....8888..");
    }

    #[test]
    fn read() {
        let value = super::read("12345").unwrap();
        assert_eq!(fmt(&value), "0..111....22222");
    }

    #[test]
    fn disk_fragmenter() {
        let mut content = super::read(&read_to_string("data/09.in").unwrap()).unwrap();
        super::defragment(&mut content);
        let value = super::checksum(&content);

        assert_eq!(value, 6353658451014);
    }
    #[test]
    fn disk_fragmenter_whole() {
        let mut content = super::read(&read_to_string("data/09.in").unwrap()).unwrap();
        super::defragment_whole(&mut content);
        let value = super::checksum(&content);

        assert_eq!(value, 6382582136592);
    }
}
