use itertools::izip;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day1_1(args: &[String]) -> Result<(), anyhow::Error> {
    if args.len() < 1 {
        panic!("Expected file name")
    }

    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for line in read_lines(&args[0])?.flatten() {
        let (id1, id2) = line
            .split_once(" ")
            .expect("numbers must be separated by ' '");
        let id1: i32 = id1.trim().parse().expect("not an iteger");
        let id2: i32 = id2.trim().parse().expect("not an iteger");

        list1.push(id1);
        list2.push(id2);
    }

    list1.sort();
    list2.sort();

    let mut sum = 0;
    for (left, right) in izip!(list1, list2) {
        sum = sum + (left - right).abs();
    }
    println!("total distance = {sum}");

    Ok(())
}

fn day1_2(args: &[String]) -> Result<(), anyhow::Error> {
    if args.len() < 1 {
        panic!("Expected file name")
    }

    let mut list1: Vec<i32> = vec![];
    let mut map2: HashMap<i32, i32> = HashMap::new();

    for line in read_lines(&args[0])?.flatten() {
        let (id1, id2) = line
            .split_once(" ")
            .expect("numbers must be separated by ' '");
        let id1: i32 = id1.trim().parse().expect("not an iteger");
        let id2: i32 = id2.trim().parse().expect("not an iteger");

        list1.push(id1);
        if let Some(value) = map2.get_mut(&id2) {
            *value += 1;
        } else {
            map2.insert(id2, 1);
        }
    }

    list1.sort();

    let mut sum = 0;
    for left in list1 {
        sum = sum + left * map2.get(&left).unwrap_or(&0);
    }
    println!("total similarity = {sum}");

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please specify day")
    }

    let _ = match args[1].as_str() {
        "day1.1" => day1_1(&args[2..]),
        "day1.2" => day1_2(&args[2..]),
        u => panic!("Unknow day {u}"),
    };
}
