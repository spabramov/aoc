use std::{
    collections::{HashMap, VecDeque},
    fs, usize,
};

pub fn read(filename: &str) -> anyhow::Result<VecDeque<usize>> {
    Ok(fs::read_to_string(filename)
        .unwrap()
        .split(' ')
        .map(|i| i.trim().parse::<usize>())
        .map_while(|x| x.ok())
        .collect())
}

pub fn blink(n: usize, value: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(cnt) = cache.get(&(n, value)) {
        *cnt
    } else if n == 0 {
        1
    } else {
        match value {
            0 => {
                let out = blink(n - 1, 1, cache);
                cache.insert((n - 1, 1), out);
                out
            }

            value if value.ilog10() % 2 == 1 => {
                let len = value.ilog10() / 2 + 1;
                let div = 10usize.pow(len);
                let out1 = blink(n - 1, value / div, cache);
                let out2 = blink(n - 1, value % div, cache);
                cache.insert((n - 1, value / div), out1);
                cache.insert((n - 1, value % div), out2);
                out1 + out2
            }
            value => {
                let out = blink(n - 1, value * 2024, cache);
                cache.insert((n - 1, value * 2024), out);
                out
            }
        }
    }
}

#[cfg(test)]
mod test {

    use std::collections::{HashMap, VecDeque};

    #[test]
    fn plutonian_pebbles_25() {
        let input: VecDeque<usize> = super::read("data/11.in").unwrap();
        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
        let value: usize = input
            .iter()
            .map(|&value| super::blink(25, value, &mut cache))
            .sum();

        assert_eq!(value, 183484);
    }

    #[test]
    fn plutonian_pebbles_75() {
        let input: VecDeque<usize> = super::read("data/11.in").unwrap();
        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
        let value: usize = input
            .iter()
            .map(|&value| super::blink(75, value, &mut cache))
            .sum();

        assert_eq!(value, 218817038947400);
    }
}
