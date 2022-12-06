#![feature(iter_array_chunks)]
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "inputs/day4";
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim().lines();

    let mut result1 = 0;

    for line in contents {
        let [range1, range2] = match line.split(',').collect::<Vec<&str>>()[..] {
            [a, b] => [a, b],
            _ => ["0", "0"],
        };

        println!("{:?} {:?}", range1, range2);
        let [r1x, r1y] = match split_parse_range(range1)[..] {
            [a, b] => [a, b],
            _ => [0, 0],
        };
        let [r2x, r2y] = match split_parse_range(range2)[..] {
            [a, b] => [a, b],
            _ => [0, 0],
        };
        if r1x <= r2x && r1y >= r2y {
            println!("range2 fits in range1??");
            result1 += 1;
        } else if r1x >= r2x && r1y <= r2y {
            println!("range1 fits in range2??");
            result1 += 1;
        } else {
            println!("NEXT!");
        }
        println!();
    }

    println!("{}", result1);

    Ok(())
}

fn split_parse_range(range_str: &str) -> Vec<i32> {
    let parsed = range_str
        .split('-')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    parsed
}
